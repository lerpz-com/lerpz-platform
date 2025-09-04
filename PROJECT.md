# Standardised Joiners, Movers, and Leavers (JML) Process for Employee Lifecycle Management

## 1. Abstract

This document describes a project to design and implement an improved Joiners,
Movers, and Leavers (JML) process for the organisation. The project’s primary
goal is to streamline the way employees and contractors are onboarded,
transition into new roles, and exit the company.

By introducing improved workflows, automation, and clearer communication between
HR, IT, and Facilities teams, the initiative aims to reduce delays, minimise
security risks, and improve overall efficiency in managing the employee
lifecycle.

## 2. Introduction

The Joiners, Movers, and Leavers (JML) process covers the key stages of an
employee’s lifecycle within the organisation — from the moment an offer is
accepted, through internal role changes, to eventual departure. In its current
state, the process involves manual steps, inconsistent communication across
departments, and a lack of unified tracking, which often results in delays,
duplicated work, and potential security gaps.

This project seeks to address these challenges through a coordinated effort to:

- Map current processes and identify bottlenecks.
- Define an optimised future-state workflow for JML activities.
- Integrate HR, IT, and Facilities functions into a seamless process.
- Increase automation for provisioning, updating, and revoking access.
- Improve visibility and tracking through a centralised system.

The outcome of this project will be a streamlined, secure, and auditable process
that supports operational efficiency, ensures compliance with data protection
requirements, and enhances the experience for joiners, movers, and leavers.

## 3. Scope

### 3.1 In Scope

This project will focus on designing, building, and implementing an enhanced JML
process that covers:

- **Process Mapping & Analysis**\
    Documenting the current “as-is” JML process across all departments.
    Identifying bottlenecks, inconsistencies, and risks in the current workflow.

- **Process Redesign**\
    Developing a standard “to-be” JML workflow that aligns with business, security, and compliance requirements.
    Outlining integration points between HR, IT, Security, and Facilities.

- **Technology & Automation**\
    Evaluating and selecting tools or systems to support automation of account provisioning, moves, and deprovisioning.
    Designing integrations between HR systems, IT service management tools, and identity/access management solutions.

- **Roles & Responsibilities Alignment**\
    Defining clear ownership for each step in the JML process.
    Creating communication channels and escalation paths.

- **Security & Compliance Controls**\
    Ensuring the new process meets data protection, audit, and access governance requirements.
    Implementing an audit trail for all JML activities.

- **Training & Change Management**\
    Preparing training materials and conducting awareness sessions for stakeholders.
    Supporting transition from the old process to the new process.

### 3.2 Out of Scope

To maintain focus and deliver within realistic timelines, the following items
will not be covered under this project:

- Major upgrades or replacements of core HR or IT systems not directly related
    to JML automation.
- Physical office redesign or large-scale facility changes.
- Broader HR process re-engineering outside of onboarding, role changes, and
    offboarding.
- Non-employee processes unrelated to contractors, interns, or temporary staff.

## 4. Definitions

To ensure clarity and consistency during the project and in the designed
process, the following terms are defined as they will be used within this document:

### 4.1 Joiner

An individual (employee, contractor, intern, or temporary staff member) who is
starting their engagement with the organisation and requires initial
provisioning of access, equipment, and workspace.

### 4.2 Mover

An existing member of the organisation who is changing roles, departments,
business units, or employment status (e.g., transitioning from contractor to
employee). Movers require adjustments to system and physical access, equipment,
and responsibilities.

### 4.3 Leaver

An individual who is ending their employment or engagement with the
organisation, voluntarily or involuntarily. This includes permanent departures,
end-of-contract scenarios, or internal transitions out of the organisation.
Leavers require revocation of access, return of equipment, and closure of
employment records.

### 4.4 Provisioning

The process of granting an individual required access to systems, applications,
data sources, facilities, and/or equipment.

### 4.5 Deprovisioning

The process of removing an individual’s access to systems, applications, data
sources, and/or facilities, as well as retrieving company-owned assets.

### 4.6 Access Management System

A digital platform or service that manages user accounts, roles, permissions,
and credentials. Often integrates with HR and IT service management systems.

### 4.7 Workflow

A defined sequence of tasks, decision points, and approvals that need to be
completed for a given process step in the JML lifecycle.

### 4.8 Stakeholder

Any individual, team, or department with a role or vested interest in the JML
process, including HR, IT, Facilities, Security, Finance, and department
managers.
