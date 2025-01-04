import React, { useState, useRef, useEffect } from "react";
import { Link, useLocation } from "react-router-dom";
import { useIdentity } from "../contexts/IdentityContext";
import ConnectDialog from "./ConnectDialog";
import "../styles/Navbar.css";

const Navbar: React.FC = () => {
  const location = useLocation();
  const { identity, logout } = useIdentity();
  const [isConnectDialogOpen, setIsConnectDialogOpen] = useState(false);
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  const isActive = (path: string) =>
    location.pathname === path ? "navbar-link active" : "navbar-link";

  const handleDisconnect = async () => {
    await logout();
    setIsDropdownOpen(false);
  };

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setIsDropdownOpen(false);
      }
    };

    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  return (
    <nav className="navbar">
      <div className="navbar-container">
        <div className="navbar-content">
          <div className="navbar-left">
            <div className="navbar-brand">
              <span className="navbar-logo">RunicSwap</span>
            </div>
            <div className="navbar-links">
              <Link to="/dashboard" className={isActive("/dashboard")}>
                Dashboard
              </Link>
              <Link to="/explore" className={isActive("/explore")}>
                Explore
              </Link>
              <Link to="/about" className={isActive("/about")}>
                About
              </Link>
            </div>
          </div>
          <div className="navbar-actions">
            {identity ? (
              <div className="navbar-dropdown" ref={dropdownRef}>
                <button
                  className="navbar-dropdown-toggle"
                  onClick={() => setIsDropdownOpen(!isDropdownOpen)}
                >
                  Connected
                </button>
                {isDropdownOpen && (
                  <div className="navbar-dropdown-menu">
                    <Link to="/portfolio" className="navbar-dropdown-item">
                      Portfolio
                    </Link>
                    <button
                      className="navbar-dropdown-item"
                      onClick={handleDisconnect}
                    >
                      Disconnect
                    </button>
                  </div>
                )}
              </div>
            ) : (
              <button
                className="navbar-connect-button"
                onClick={() => setIsConnectDialogOpen(true)}
              >
                Connect
              </button>
            )}
          </div>
        </div>
      </div>
      <ConnectDialog
        isOpen={isConnectDialogOpen}
        setIsOpen={setIsConnectDialogOpen}
      />
    </nav>
  );
};

export default Navbar;
