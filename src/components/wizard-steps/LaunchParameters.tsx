import React from "react";
import type { StartLaunchArgs } from "./../../declarations/backend/backend.did.d.ts";

interface LaunchParametersProps {
  formData: StartLaunchArgs;
  updateFormData: (newData: Partial<StartLaunchArgs>) => void;
}

const LaunchParameters: React.FC<LaunchParametersProps> = ({
  formData,
  updateFormData,
}) => {
  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value, type } = e.target;
    let newValue: any = value;

    if (type === "number") {
      newValue = parseFloat(value);
      if (name === "soft_cap" || name === "hard_cap") {
        newValue = BigInt(Math.floor(newValue * 1e8));
      } else if (name === "duration" || name === "starts_in") {
        newValue = Math.floor(newValue * 24 * 60 * 60); // Convert days to seconds
      }
    } else if (type === "checkbox") {
      newValue = e.target.checked;
    }

    updateFormData({ [name]: newValue });
  };

  return (
    <div className="wizard-step-content">
      <h2>Launch Parameters</h2>
      <div className="form-group">
        <label htmlFor="soft_cap">Soft Cap* (in BTC)</label>
        <input
          type="number"
          id="soft_cap"
          name="soft_cap"
          value={Number(formData.soft_cap) / 1e8}
          onChange={handleInputChange}
          required
          min="0"
          step="any"
          placeholder="Enter soft cap"
        />
      </div>
      <div className="form-group">
        <label htmlFor="hard_cap">Hard Cap* (in BTC)</label>
        <input
          type="number"
          id="hard_cap"
          name="hard_cap"
          value={Number(formData.hard_cap) / 1e8}
          onChange={handleInputChange}
          required
          min="0"
          step="any"
          placeholder="Enter hard cap"
        />
      </div>
      <div className="form-group">
        <label htmlFor="duration">Duration* (in days)</label>
        <input
          type="number"
          id="duration"
          name="duration"
          value={formData.duration / (24 * 60 * 60)}
          onChange={handleInputChange}
          required
          min="0"
          step="any"
          placeholder="Enter duration in days"
        />
      </div>
      <div className="form-group">
        <label htmlFor="starts_in">Starts In* (in days)</label>
        <input
          type="number"
          id="starts_in"
          name="starts_in"
          value={formData.starts_in / (24 * 60 * 60)}
          onChange={handleInputChange}
          required
          min="0"
          step="any"
          placeholder="Enter start time in days"
        />
      </div>
      <div className="form-group checkbox-group">
        <input
          type="checkbox"
          id="turbo"
          name="turbo"
          checked={formData.turbo}
          onChange={handleInputChange}
        />
        <label htmlFor="turbo">Turbo</label>
      </div>
    </div>
  );
};

export default LaunchParameters;
