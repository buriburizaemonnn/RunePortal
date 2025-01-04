import React, { useCallback, useEffect } from "react";
import type { StartLaunchArgs } from "./../../declarations/backend/backend.did.d.ts";

interface ProjectDetailsProps {
  formData: StartLaunchArgs;
  updateFormData: (newData: Partial<StartLaunchArgs>) => void;
}

const ProjectDetails: React.FC<ProjectDetailsProps> = ({
  formData,
  updateFormData,
}) => {
  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    updateFormData({ [name]: [value] });
    console.log(`Updated ${name}: ${value}`);
  };

  const handleImageUpload = useCallback(
    (e: React.ChangeEvent<HTMLInputElement>) => {
      const file = e.target.files?.[0];
      if (file) {
        if (file.size > 1024 * 1024) {
          alert("File size should not exceed 1MB");
          return;
        }

        const reader = new FileReader();
        reader.onload = (event) => {
          const arrayBuffer = event.target?.result as ArrayBuffer;
          const uint8Array = new Uint8Array(arrayBuffer);
          updateFormData({
            logo: [uint8Array],
            content_type: [new TextEncoder().encode(file.type)],
          });

          // Log image details
          console.log("Image uploaded:");
          console.log("- Name:", file.name);
          console.log("- Size:", file.size, "bytes");
          console.log("- Type:", file.type);
          console.log(
            "- Type as byte array:",
            Array.from(new TextEncoder().encode(file.type)),
          );
          console.log(
            "- First 50 bytes of image:",
            Array.from(uint8Array.slice(0, 50)),
          );
        };
        reader.readAsArrayBuffer(file);
      }
    },
    [updateFormData],
  );

  useEffect(() => {
    console.log("ProjectDetails component rendered");
    return () => {
      console.log("ProjectDetails component unmounted");
    };
  }, []);

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
          <div className="image-preview">
            <img
              src={URL.createObjectURL(blob)}
              alt="Uploaded logo"
              style={{
                maxWidth: "200px",
                maxHeight: "200px",
                marginTop: "10px",
              }}
            />
          </div>
        );
      } catch (error) {
        console.error("Error creating image preview:", error);
        return <div>Error creating image preview</div>;
      }
    }
    return null;
  };

  return (
    <div className="wizard-step-content">
      <h2>Project Details</h2>
      <div className="form-group">
        <label htmlFor="logo">Logo (max 1MB)</label>
        <input
          type="file"
          id="logo"
          name="logo"
          accept="image/*"
          onChange={handleImageUpload}
        />
        {renderImagePreview()}
      </div>
      <div className="form-group">
        <label htmlFor="website">Website</label>
        <input
          type="url"
          id="website"
          name="website"
          value={formData.website[0] || ""}
          onChange={handleInputChange}
          placeholder="https://example.com"
        />
      </div>
      <div className="form-group">
        <label htmlFor="telegram">Telegram</label>
        <input
          type="url"
          id="telegram"
          name="telegram"
          value={formData.telegram[0] || ""}
          onChange={handleInputChange}
          placeholder="https://t.me/yourgroup"
        />
      </div>
      <div className="form-group">
        <label htmlFor="openchat">OpenChat</label>
        <input
          type="url"
          id="openchat"
          name="openchat"
          value={formData.openchat[0] || ""}
          onChange={handleInputChange}
          placeholder="https://oc.app/yourgroup"
        />
      </div>
    </div>
  );
};

export default ProjectDetails;
