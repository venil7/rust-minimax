import classnames from 'classnames';
import { Field, Game, Player } from "darkruby-tictactoe";
import * as React from 'react';
import { useEffect, useReducer } from 'react';
import * as ReactDOM from "react-dom";

const { Cross, Empty, Nought } = Field;
const { None, CPU, Human } = Player;

const initState = () => ({
  game: null,
  cells: Array(9).fill(Empty),
  winner: Player.None,
});

const updateArray = (array, idx, val) => {
  const copy = [...array];
  copy[idx] = val;
  return copy;
};

const reducer = (state, action) => {
  switch (action.type) {
    case 'SET_GAME': return { ...state, game: action.game };
    case 'CPU_MOVE': return {
      ...state,
      game: action.game,
      winner: action.winner,
      cells: updateArray(state.cells, action.idx, Nought)
    };
    case 'HUMAN_MOVE': return {
      ...state,
      cells: updateArray(state.cells, action.idx, Cross)
    };
    default:
      return { ...state };
  }
};

const Winner = ({ winner }) => {
  switch (winner) {
    case CPU: return (<h1>
      <span class="nought">CPU</span> <span class="cross">WINS</span>
    </h1>);
    case Human: return (<h1>
      <span class="nought">HUMAN</span> <span class="cross">WINS</span>
    </h1>); //unlikely :)
    default:
      return null;
  }
};

const Cell = ({ field, onClick, winner }) => {
  const c = classnames({
    'blank': field === Empty,
    'nought': field === Nought,
    'cross': field === Cross,
  });
  const clickHandler = (field === Empty && winner === None)
    ? onClick
    : () => void 0;

  return (
    <li className={c} onClick={() => clickHandler()} />
  )
};

const App = (props) => {
  const [state, dispatch] = useReducer(reducer, initState());
  const { game, cells, winner } = state;

  useEffect(() => {
    if (!game) {
      const g = Game.new();
      dispatch({ type: 'SET_GAME', game: g });
    }
  }, [game]);

  const makeMove = (idx) => {
    dispatch({ type: 'HUMAN_MOVE', idx });
    setTimeout(() => state.game.make_move(idx, Cross, (cpuIdx, winner, game) => {
      dispatch({
        type: 'CPU_MOVE',
        idx: cpuIdx,
        game, winner
      });
    }), 100);
  }
  return (
    <>
      <ul className="game">
        {state.cells.map((cell, idx) => (
          <Cell
            key={idx}
            winner={winner}
            field={cell}
            onClick={() => makeMove(idx)}
          />
        ))}
      </ul>
      <Winner winner={winner} />
    </>
  );
};

const root = document.getElementById("app");
ReactDOM.render(<App />, root);
