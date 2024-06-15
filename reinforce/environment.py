import logging
import numpy as np
import gymnasium as gym
import torch.optim as optim
import torch.nn as nn

from gymnasium import spaces
from collections import deque
from typing import Dict, List

from reinforce.sampler import DataSampler
from reinforce.model import PolicyNet, select_action, compute_loss
from reinforce.utils import Position, compute_sharpe_ratio


class TradeEnv(gym.Env):
    def __init__(
            self, 
            state_dim: int, 
            action_dim: int, 
            embedding_dim: int,
            queue_size: int,
            inaction_cost: float,
            action_cost: float,
            device: str,
            db_path: str,
            return_thresh: float,
            feature_params: Dict[str, List[int] | Dict[str, List[int]]]={
                "sma": np.geomspace(8, 64, 8).astype(int).tolist(),
                "ema": np.geomspace(8, 64, 8).astype(int).tolist(),
                "rsi": np.geomspace(4, 64, 8).astype(int).tolist(),
                "sto": {
                    "window": np.geomspace(8, 64, 4).astype(int).tolist() + np.geomspace(8, 64, 4).astype(int).tolist(),
                    "k": np.linspace(3, 11, 8).astype(int).tolist(),
                    "d": np.linspace(3, 11, 8).astype(int).tolist()
                }
            }) -> None:
        super().__init__()

        self.action_space = spaces.Discrete(action_dim)
        self.observation_space = spaces.Discrete(state_dim)

        self._device = device
        self._sampler = DataSampler(db_path, queue_size, feature_params=feature_params)
        self._policy_net = PolicyNet(state_dim, action_dim, len(Position), embedding_dim).to(device)
        self._return_thresh = return_thresh
        self._position = Position.Cash
        self._inaction_cost = inaction_cost
        self._action_cost = action_cost
        self._portfolio = 1.0
        self._returns = []
        self._log_probs = []
        self._init_close = 0.0
        self._risk_free_rate = 1.0
        self._entry = 0.0
        self._exit = 0.0

    @property
    def model(self):
        return self._policy_net

    @property
    def model_weights(self):
        return self._policy_net.state_dict()

    @property
    def log_probs(self):
        return self._log_probs
    
    @property
    def portfolio(self):
        return self._portfolio
    
    @property
    def init_close(self):
        return self._init_close

    def reset(self):
        self._sampler.reset()
        _, close, state = self._sampler.sample_next()

        while len(state) == 0:
            _, close, state = self._sampler.sample_next()

        self._position = Position.Cash
        self._portfolio = 1.0
        self._returns = []
        self._log_probs = []
        self._init_close = close
        self._risk_free_rate = 1.0
        self._entry = 0.0
        self._exit = 0.0
        return state

    def step(self, action: int):
        end, close, state = self._sampler.sample_next()
        reward, done = self._inaction_cost, False

        # Valid buy
        if self._position == Position.Cash and action == 0:
            self._position = Position.Asset
            self._entry = close
            self._exit = 0.0
            self._portfolio *= (1-self._action_cost)
        
        # Valid sell
        elif self._position == Position.Asset and action == 2:
            self._position = Position.Cash
            self._exit = close
            gross_return = close / self._entry
            net_return = gross_return * (1 - self._action_cost)  # Adjust return by action cost
            self._returns.append(net_return)  # Store the net return (subtracting 1 to get the actual return)
            self._portfolio *= net_return
            if self._portfolio < self._return_thresh:
                logging.info("portfolio threshold hit, done")
                done = True
            
            self._risk_free_rate = close / self._init_close
            self._entry = 0.0
            reward += compute_sharpe_ratio(self._returns, self._risk_free_rate)

        action, log_prob = select_action(
            self._policy_net, 
            state, 
            int(self._position.value), 
            self._device)
        
        self._log_probs += [log_prob]

        if end: done = True

        return action, reward, done, False, {"price": close}

def train(
        env: TradeEnv, 
        episodes: int, 
        learning_rate: float=1e-3, 
        momentum: float=1e-3,
        max_grad_norm: float=1.0,
        portfolio_size: int=5) -> List[float]:
    logging.info("training starts")
    optimizer = optim.SGD(
        env.model.parameters(), 
        lr=learning_rate, 
        momentum=momentum)

    buy_and_hold = None
    reward_history = []
    portfolios = deque(maxlen=portfolio_size)

    for e in range(episodes):
        logging.info(f"episode {e+1}/{episodes} began")
        _ = env.reset()
        env.model.train()
        action, reward, done, _, close = env.step(1)
        rewards = [reward]
        while not done:
            action, reward, done, _, close = env.step(action)
            rewards += [reward]

        optimizer.zero_grad()
        loss = compute_loss(env.log_probs, rewards, device=env._device)
        loss.backward()
        nn.utils.clip_grad_norm_(env.model.parameters(), max_grad_norm)
        optimizer.step()

        reward_history += [sum(rewards)]
        portfolios += [env.portfolio]

        if not buy_and_hold:
            buy_and_hold = (close["price"] / env.init_close - 1)

        logging.info(f"""\n
        episode {e+1}/{episodes} done
        loss:            {' ' if loss.item() > 0 else ''}{loss.item():.4f}
        model portfolio: {'+' if env.portfolio > 1 else ''}{(env.portfolio-1) * 100:.4f}%
        buy and hold:    {'+' if buy_and_hold > 0 else ''}{buy_and_hold * 100:.4f}%
        """)
        env.model.eval()

        avg_port = np.mean(portfolios) - 1
        if avg_port > buy_and_hold and portfolios[-1] > buy_and_hold and len(portfolios) == portfolio_size:
            logging.info(f"average target reached, last {portfolio_size} averaged {'+' if avg_port > 1 else ''}{avg_port * 100:.4f}%, exiting.")
            return reward_history

    logging.info(f"training complete, last {portfolio_size} averaged {'+' if avg_port > 1 else ''}{avg_port * 100:.4f}%")
    return reward_history
