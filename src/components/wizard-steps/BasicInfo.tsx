import React from "react";
import type { StartLaunchArgs } from "./../../declarations/backend/backend.did.d.ts";

interface BasicInfoProps {
  formData: StartLaunchArgs;
  updateFormData: (newData: Partial<StartLaunchArgs>) => void;
}

const BasicInfo: React.FC<BasicInfoProps> = ({ formData, updateFormData }) => {
  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value, type } = e.target;
    let newValue: any = value;

    if (type === "number") {
      newValue = parseFloat(value);
      if (name === "total_supply" || name === "price_per_token") {
        newValue = BigInt(Math.floor(newValue * 1e8));
      }
    }

    updateFormData({ [name]: newValue });
  };

  return (
    <div className="wizard-step-content">
      <h2>Basic Information</h2>
      <div className="form-group">
        <label htmlFor="runename">Rune Name*</label>
        <input
          type="text"
          id="runename"
          name="runename"
          value={formData.runename}
          onChange={handleInputChange}
          required
          placeholder="Enter your Rune name"
        />
      </div>
      <div className="form-group">
        <label htmlFor="total_supply">Total Supply*</label>
        <input
          type="number"
          id="total_supply"
          name="total_supply"
          value={Number(formData.total_supply) / 1e8}
          onChange={handleInputChange}
          required
          min="0"
          step="any"
          placeholder="Enter total supply"
        />
      </div>
      <div className="form-group">
        <label htmlFor="price_per_token">Price Per Token* (in BTC)</label>
        <input
          type="number"
          id="price_per_token"
          name="price_per_token"
          value={Number(formData.price_per_token) / 1e8}
          onChange={handleInputChange}
          required
          min="0"
          step="any"
          placeholder="Enter price per token"
        />
      </div>
      <div className="form-group">
        <label htmlFor="divisibility">Divisibility*</label>
        <input
          type="number"
          id="divisibility"
          name="divisibility"
          value={formData.divisibility}
          onChange={handleInputChange}
          required
          min="0"
          step="1"
          placeholder="Enter divisibility"
        />
      </div>
    </div>
  );
};

export default BasicInfo;
