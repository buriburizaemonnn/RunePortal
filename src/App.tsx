import "./App.css";
import Home from "./routes/Home";
// import { IdentityProvider } from "./contexts/IdentityContext";
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import Launch from "./routes/Launch";
// import Navbar from "./components/Navbar";

function App() {
  return (
    // <IdentityProvider>
    <Router>
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/launch" element={<Launch />} />
      </Routes>
    </Router>
    // </IdentityProvider>
  );
}

export default App;
