import React from 'react';
import { Routes, Route } from 'react-router-dom';
import DraggableCube from './components/DraggableCube.tsx';
import RubiksCube2D from './components/RubiksCube2D.tsx';

const App: React.FC = () => {
  return (
    <div className="container">
      <Routes>
        <Route path="/" element={<RubiksCube2D />} />
      </Routes>
    </div>
  );
};

export default App;