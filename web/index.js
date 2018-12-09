import { Field, Game, Player } from "darkruby-tictactoe";
import * as React from 'react';
import { useEffect, useReducer } from 'react';
import * as ReactDOM from "react-dom";

const { Cross, Empty, Nought } = Field;

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
    case 'SET_WINNER': return { ...state, winner: action.winner };
    case 'SET_MOVE': return { ...state, cells: updateArray(state.cells, action.idx, action.field) };
  }
};

const toField = (f) => {
  switch (f) {
    case Field.Cross: return "X";
    case Field.Nought: return "O";
    case Field.Empty: return "";
  }
};

const Cell = ({ field, onClick }) => (
  <div className="cell" onClick={() => onClick()}>
    {toField(field)}
  </div>
);

const App = (props) => {
  const [state, dispatch] = useReducer(reducer, initState());
  const { game, cells, winner } = state;

  useEffect(() => {
    if (!game) {
      const g = Game.new(
        (idx) => dispatch({ type: 'SET_MOVE', idx, field: Field.Nought }),
        (winner) => dispatch({ type: 'SET_WINNER', winner })
      );
      dispatch({ type: 'SET_GAME', game: g });
    }
  }, [game]);

  const makeMove = (idx) => {
    dispatch({ type: 'SET_MOVE', idx, field: Field.Cross });
    setTimeout(() => state.game.make_move(idx, Cross), 100);
  }
  return (
    <div className="board">
      {state.cells.map((cell, idx) => (
        <Cell
          key={idx}
          field={cell}
          onClick={() => makeMove(idx)}
        />
      ))}
    </div>
  );
};

const root = document.getElementById("app");
ReactDOM.render(<App />, root);
