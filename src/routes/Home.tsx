import React from "react";
import { Link } from "react-router-dom";
import "../styles/Home.css";

const Home: React.FC = () => {
  return (
    <div className="home-container">
      <header>
        <h1 className="home-title">RunePortal</h1>
      </header>

      <p className="home-tagline">
        Your Gateway to Launching and Discovering Rune Projects
      </p>

      <div className="home-buttons">
        <Link to="/launch" className="home-button home-button-launch">
          Launch Your Project
        </Link>
        <Link to="/projects" className="home-button home-button-view">
          View Projects
        </Link>
      </div>
    </div>
  );
};

export default Home;
