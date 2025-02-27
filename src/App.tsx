import React from 'react';
import { Routes, Route } from 'react-router-dom';
import DraggableCube from './components/DraggableCube.tsx';

const App: React.FC = () => {
  return (
    <div className="container">
      <Routes>
        <Route path="/" element={<DraggableCube />} />
      </Routes>
    </div>
  );
};

export default App;