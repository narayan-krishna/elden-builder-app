import { BrowserRouter, Routes, Route } from "react-router-dom"
import './App.css';
import Home from './pages/Home'

// <Route path="/dashboard" element={<Dashboard authed={true} />} />

function App() {

  // TODO: make this its own page (setup/configuration) and make weapon and stat state global
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Home />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
