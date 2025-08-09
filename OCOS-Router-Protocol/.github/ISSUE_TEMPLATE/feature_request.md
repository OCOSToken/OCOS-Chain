---
name: "💡 Feature Request"
about: "Propose a new function, enhancement, or improvement for the protocol."
title: "[Feature] Short descriptive title"
labels: ["feature", "enhancement"]
assignees: []
---

## 📌 Summary
A concise one- or two-sentence summary of the new feature or enhancement.  
> Example: "Add LayerZero adapter support to RouterCore so that cross-chain operations can be executed via LayerZero."

---

## 🎯 Problem Statement & Rationale
What problem does this feature solve and why is it important?  
- How will it improve the user or developer experience?  
- What limitations or gaps exist in the current setup?  
- What happens if we don’t implement this feature?

---

## 🛠 Proposed Solution
Describe how the feature should work, including technical or functional details.  
- Provide a step-by-step scenario if possible.  
- If this requires integration with existing APIs, adapters, or modules, specify which ones.  

> **Example:**  
> 1. Implement `IBridgeAdapter` interface in `contracts/adapters/LayerZeroAdapter.sol`.  
> 2. Add the adapter to the allowlist via `RouterCore.setAdapter()`.  
> 3. Write an end-to-end test script for a successful testnet transaction.

---

## 📂 Affected Files & Modules
- Which files, folders, or modules will be impacted?  
- If new files will be created, list their names and suggested locations.

---

## 📊 Acceptance Criteria
How will we know this feature is complete and successful?  
- ✅ All unit tests pass  
- ✅ Successfully tested on a public testnet after deployment  
- ✅ No regressions or unintended side effects

---

## ⚠️ Risks & Challenges
- Potential security risks (e.g., cross-chain replay attacks, adapter validation issues)  
- Architectural implications  
- Additional audit requirements

---

## 📅 Estimated Time & Resources
- Estimated implementation time: `X days/weeks`  
- Required resources: (developer time, test environments, API keys, etc.)

---

## 🔗 Additional References
Add any supporting documentation, links, or visual assets here.  
> Example: “LayerZero Docs” → https://docs.layerzero.network/

---

**Note:** Please be as specific and technically detailed as possible so the development team can accurately evaluate and plan the implementation.
