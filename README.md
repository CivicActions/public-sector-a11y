# A11yWatch + CivicActions Public Sector Accessibility

_Proof of concept for large scale, government scans_

## Overview

### Project Goals

- Define standardized accessibility metrics
- Regularly report on government web property accessability
- Open source all work so others can recreate and help the project
- Spread awareness of accessibility as it relates to government agencies

## Inspiration & Resources

_To-Do: Move below to /docs folder_

### Infrastructure

Cloud resources are managed by the .yaml files in the /k8s directory. The namespace `public-sector-a11y` has been assigned to resources orchestrated from this repo. The `metadata.name` value in the k8 files defines the sub-domain of the public url.

For example, the `a11ywatch-frontend` file deploys resources avaliable at [a11ywatch-frontend.public-sector-a11y.app.civicactions.net](https://a11ywatch-frontend.public-sector-a11y.app.civicactions.net/)
