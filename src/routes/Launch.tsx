import React, { useState, useCallback } from "react";
import LaunchWizard from "../components/LaunchWizard";
import "../styles/Launch.css";
import type { StartLaunchArgs } from "../declarations/backend/backend.did.d.ts";

const Launch: React.FC = () => {
  const [currentStep, setCurrentStep] = useState(1);
  const [formData, setFormData] = useState<StartLaunchArgs>({
    x: [],
    fee_per_vbytes: [],
    duration: 0,
    turbo: false,
    starts_in: 0,
    logo: [],
    content_type: [],
    divisibility: 0,
    hard_cap: BigInt(0),
    website: [],
    price_per_token: BigInt(0),
    soft_cap: BigInt(0),
    raise_in: { Bitcoin: null },
    runename: "",
    telegram: [],
    total_supply: BigInt(0),
    symbol: [],
    openchat: [],
  });

  const updateFormData = useCallback((newData: Partial<StartLaunchArgs>) => {
    setFormData((prevData) => ({ ...prevData, ...newData }));
  }, []);

  const nextStep = useCallback(
    () => setCurrentStep((prev) => Math.min(prev + 1, 4)),
    [],
  );
  const prevStep = useCallback(
    () => setCurrentStep((prev) => Math.max(prev - 1, 1)),
    [],
  );

  return (
    <div className="launch-container">
      <h1 className="launch-title">Launch Your Rune Project</h1>
      <p className="launch-description">
        Create and launch your own Rune token in just a few simple steps.
      </p>
      <LaunchWizard
        currentStep={currentStep}
        formData={formData}
        updateFormData={updateFormData}
        nextStep={nextStep}
        prevStep={prevStep}
      />
    </div>
  );
};

export default Launch;
