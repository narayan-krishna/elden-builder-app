import { BrowserRouter, Routes, Route } from "react-router-dom"
import './App.css';
import Home from './pages/Home'
import SignUpPage from './pages/SignUpPage'

// <Route path="/dashboard" element={<Dashboard authed={true} />} />

function App() {

  // TODO: make this its own page (setup/configuration) and make weapon and stat state global
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/elden-builder-app/" element={<Home />} />
        <Route path="/signup-test/" element={<SignUpPage />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
