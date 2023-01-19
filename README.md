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

### API

[**OpenAPI 3.0.9 Schema**](https://raw.githubusercontent.com/a11ywatch/a11ywatch/main/clients/src/schema/api.json)

[API Docs](https://a11ywatch.com/api-info)

#### Schema Docs

[**JavaScript**](https://gitlab.com/j-mendez/a11ywatch-clients/-/tree/main/javascript_api_client)

##### APIs

##### Collection API

[Collection API Docs](https://gitlab.com/j-mendez/a11ywatch-clients/-/blob/main/javascript_api_client/docs/CollectionApi.md)

| Method                                            | HTTP request            | Description                                         |
| ------------------------------------------------- | ----------------------- | --------------------------------------------------- |
| [**getAnalytics**](CollectionApi.md#getAnalytics) | **GET** /list/analytics | Get the analytics for a website                     |
| [**getIssues**](CollectionApi.md#getIssues)       | **GET** /list/issue     | List the issues for a website                       |
| [**getPageSpeed**](CollectionApi.md#getPageSpeed) | **GET** /list/pagespeed | Get the pagespeed for a website                     |
| [**getPages**](CollectionApi.md#getPages)         | **GET** /list/pages     | List the pages in order for a website               |
| [**getWebsites**](CollectionApi.md#getWebsites)   | **GET** /list/website   | Returns websites for the user in alphabetical order |

##### Reports API

[Collection API Docs](https://gitlab.com/j-mendez/a11ywatch-clients/-/blob/main/javascript_api_client/docs/ReportsApi.md)

| Method                                                     | HTTP request            | Description                                                           |
| ---------------------------------------------------------- | ----------------------- | --------------------------------------------------------------------- | --- |
| [**crawlWebsiteStream**](ReportsApi.md#crawlWebsiteStream) | **POST** /crawl         | Multi-page crawl a website streaming issues on found                  |
| [**crawlWebsitesSync**](ReportsApi.md#crawlWebsitesSync)   | **POST** /websites-sync | Multi-page crawl all websites attached to account                     |
| [**getReport**](ReportsApi.md#getReport)                   | **GET** /report         | Get the report from a previus scan                                    |
| [**scanWebsite**](ReportsApi.md#scanWebsite)               | **POST** /scan          | Scan a website for issues                                             |     |
| [**scanWebsiteSimple**](ReportsApi.md#scanWebsiteSimple)   | **POST** /scan-simple   | Scan a website for issues without storing data and limited responses. |

##### User API

[User API Docs](https://gitlab.com/j-mendez/a11ywatch-clients/-/blob/main/javascript_api_client/docs/UserApi.md)

| Method                                  | HTTP request       | Description                             |
| --------------------------------------- | ------------------ | --------------------------------------- |
| [**createUser**](UserApi.md#createUser) | **POST** /register | Register user into the system           |
| [**getUsers**](UserApi.md#getUsers)     | **GET** /user      | Get user                                |
| [**loginUser**](UserApi.md#loginUser)   | **POST** /login    | Logs user into the system               |
| [**logoutUser**](UserApi.md#logoutUser) | **POST** /logout   | Logs out current logged in user session |

##### Websites API

[Websites API Docs](https://gitlab.com/j-mendez/a11ywatch-clients/-/blob/main/javascript_api_client/docs/WebsitesApi.md)

| Method                                                      | HTTP request        | Description                                    |
| ----------------------------------------------------------- | ------------------- | ---------------------------------------------- |
| [**addWebsite**](WebsitesApi.md#addWebsite)                 | **POST** /website   | Add a website in the collection with form data |
| [**deleteWebsite**](WebsitesApi.md#deleteWebsite)           | **DELETE** /website | Deletes a website                              |
| [**getWebsiteByDomain**](WebsitesApi.md#getWebsiteByDomain) | **GET** /website    | Find website by Domain                         |
