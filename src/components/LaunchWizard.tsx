import React, { useState, useCallback, useEffect } from "react";
import type { StartLaunchArgs } from "../declarations/backend/backend.did.d.ts";
import BasicInfo from "./wizard-steps/BasicInfo";
import LaunchParameters from "./wizard-steps/LaunchParameters";
import ProjectDetails from "./wizard-steps/ProjectDetails";
import Review from "./wizard-steps/Review";
import "../styles/LaunchWizard.css";

interface LaunchWizardProps {
  currentStep: number;
  formData: StartLaunchArgs;
  updateFormData: (newData: Partial<StartLaunchArgs>) => void;
  nextStep: () => void;
  prevStep: () => void;
}

const LaunchWizard: React.FC<LaunchWizardProps> = ({
  currentStep,
  formData,
  updateFormData,
  nextStep,
  prevStep,
}) => {
  const [completedSteps, setCompletedSteps] = useState<number[]>([]);

  useEffect(() => {
    console.log("LaunchWizard rendered. Current step:", currentStep);
    console.log("Completed steps:", completedSteps);
    console.log("Current form data:", formData);
  }, [currentStep, completedSteps, formData]);

  const renderStep = () => {
    console.log("Rendering step:", currentStep);
    switch (currentStep) {
      case 1:
        return (
          <BasicInfo formData={formData} updateFormData={updateFormData} />
        );
      case 2:
        return (
          <LaunchParameters
            formData={formData}
            updateFormData={updateFormData}
          />
        );
      case 3:
        return (
          <ProjectDetails formData={formData} updateFormData={updateFormData} />
        );
      case 4:
        return <Review formData={formData} />;
      default:
        console.error("Invalid step:", currentStep);
        return null;
    }
  };

  const handleNextStep = useCallback(() => {
    console.log("Moving to next step");
    if (!completedSteps.includes(currentStep)) {
      setCompletedSteps((prev) => {
        console.log("Updating completed steps:", [...prev, currentStep]);
        return [...prev, currentStep];
      });
    }
    nextStep();
  }, [currentStep, completedSteps, nextStep]);

  const mockApiCall = useCallback(async (data: StartLaunchArgs) => {
    console.log("Mocking API call with data:", data);
    // Simulating an API call with a 2-second delay
    await new Promise((resolve) => setTimeout(resolve, 2000));

    // Mocking a successful response
    const response = {
      success: true,
      message: "Launch created successfully",
      launchId: Math.random().toString(36).substr(2, 9),
    };
    console.log("API response:", response);
    return response;
  }, []);

  const handleSubmit = useCallback(async () => {
    console.log("Submitting form data");
    try {
      const response = await mockApiCall(formData);
      if (response.success) {
        console.log("Launch created successfully");
        alert(`Launch created successfully! Launch ID: ${response.launchId}`);
      } else {
        throw new Error("Failed to create launch");
      }
    } catch (error) {
      console.error("Error creating launch:", error);
      alert("Failed to create launch. Please try again.");
    }
  }, [formData, mockApiCall]);

  return (
    <div className="launch-wizard">
      <div className="wizard-progress">
        {[1, 2, 3, 4].map((step) => (
          <div
            key={step}
            className={`wizard-step ${currentStep >= step ? "active" : ""} ${completedSteps.includes(step) ? "completed" : ""}`}
          >
            <div className="wizard-step-inner">{step}</div>
          </div>
        ))}
      </div>
      <div className="wizard-content">{renderStep()}</div>
      <div className="wizard-navigation">
        {currentStep > 1 && (
          <button className="wizard-button prev-button" onClick={prevStep}>
            Previous
          </button>
        )}
        {currentStep < 4 && (
          <button
            className="wizard-button next-button"
            onClick={handleNextStep}
          >
            Next
          </button>
        )}
        {currentStep === 4 && (
          <button
            className="wizard-button submit-button"
            onClick={handleSubmit}
          >
            Submit
          </button>
        )}
      </div>
    </div>
  );
};

export default LaunchWizard;
