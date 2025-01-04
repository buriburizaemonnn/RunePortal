import React from "react";
import type { StartLaunchArgs } from "./../../declarations/backend/backend.did.d.ts";

interface ReviewProps {
  formData: StartLaunchArgs;
}

const Review: React.FC<ReviewProps> = ({ formData }) => {
  const renderImagePreview = () => {
    if (
      formData.logo[0] instanceof Uint8Array &&
      formData.content_type[0] instanceof Uint8Array
    ) {
      try {
        const blob = new Blob([formData.logo[0]], {
          type: new TextDecoder().decode(formData.content_type[0]),
        });
        return (
          <div className="logo-preview">
            {/*
            <img
              src={URL.createObjectURL(blob)}
              alt="Project logo"
              className="logo-image circular"
            />
	    */}
            <img
              src={URL.createObjectURL(blob)}
              alt="Project logo"
              className="logo-image square"
            />
          </div>
        );
      } catch (error) {
        console.error("Error creating image preview:", error);
        return <div>Error creating image preview</div>;
      }
    }
    return <div>No logo uploaded</div>;
  };

  return (
    <div className="wizard-step-content">
      <h2>Review Your Launch</h2>
      <div className="review-section">
        <h3>Project Logo</h3>
        {renderImagePreview()}
      </div>
      <div className="review-section">
        <h3>Basic Information</h3>
        <p>
          <strong>Rune Name:</strong> {formData.runename}
        </p>
        <p>
          <strong>Total Supply:</strong> {Number(formData.total_supply) / 1e8}
        </p>
        <p>
          <strong>Price Per Token:</strong>{" "}
          {Number(formData.price_per_token) / 1e8} BTC
        </p>
        <p>
          <strong>Divisibility:</strong> {formData.divisibility}
        </p>
      </div>
      <div className="review-section">
        <h3>Launch Parameters</h3>
        <p>
          <strong>Soft Cap:</strong> {Number(formData.soft_cap) / 1e8} BTC
        </p>
        <p>
          <strong>Hard Cap:</strong> {Number(formData.hard_cap) / 1e8} BTC
        </p>
        <p>
          <strong>Duration:</strong> {formData.duration / (24 * 60 * 60)} days
        </p>
        <p>
          <strong>Starts In:</strong> {formData.starts_in / (24 * 60 * 60)} days
        </p>
        <p>
          <strong>Turbo:</strong> {formData.turbo ? "Yes" : "No"}
        </p>
      </div>
      <div className="review-section">
        <h3>Project Details</h3>
        <p>
          <strong>Website:</strong> {formData.website[0] || "Not provided"}
        </p>
        <p>
          <strong>Telegram:</strong> {formData.telegram[0] || "Not provided"}
        </p>
        <p>
          <strong>OpenChat:</strong> {formData.openchat[0] || "Not provided"}
        </p>
      </div>
    </div>
  );
};

export default Review;
