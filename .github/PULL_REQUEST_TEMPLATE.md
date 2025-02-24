## 📝 Summary

<!-- Provide a concise description of the changes introduced in this PR. -->
This PR [adds/modifies/deletes] the `<Signal ID>` signal in the registry.

## 🔄 Changes Introduced

<!-- Detail the specific changes made in this PR. -->
- [Added/Modified/Deleted] `<Signal ID>`:
    - **Source ID:** `<source_id>`, **ID:** `<id>`, **Routes:** `<routes>` (if applicable)
- [If applicable] Added/Updated prefix configuration in `prefix/<prefix>.yaml`.
- [If applicable] Removed `<Signal ID>` and ensured no unresolved dependencies.

## 📌 Band Signaling Hub Proposal

<!-- Mandatory: Provide the link to the Band Signaling Hub Proposal forum post that supports this PR. -->
📎 **Proposal Link:** [Insert Forum Link Here]

## ✅ Checklist

<!-- Ensure all necessary tasks have been completed. -->
- [ ] Provided a valid **Band Signaling Hub Proposal** link.
- [ ] Added/Updated the signal file in the appropriate `signals/<prefix>/` directory.
- [ ] Verified that any referenced prerequisite signals exist in the registry.
- [ ] Ensured no circular dependencies are introduced.
- [ ] [If applicable] Updated prefix configuration in `prefix/<prefix>.yaml`.
- [ ] [If applicable] Updated documentation to reflect the new/modified signal.
- [ ] [If applicable] **For deleted signals**: Verified that the deleted signal is **not** a dependency for other signals, or resolved any affected dependencies.
