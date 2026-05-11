import React from "react";
import { BrowserRouter, Routes, Route } from "react-router-dom";

import { LandingPage } from "./pages/LandingPage";
import { OverviewPage } from "./pages/OverviewPage";
import { PrivacyPage } from "./pages/PrivacyPage";
import { LeadershipPage } from "./pages/LeadershipPage";
import { ProofOfLeadershipPage } from "./pages/ProofOfLeadershipPage";
import { SimulatorPage } from "./pages/SimulatorPage";
import { DocsPage } from "./pages/DocsPage";
import { ExamplesPage } from "./pages/ExamplesPage";
import { RoadmapPage } from "./pages/RoadmapPage";
import { ChangelogPage } from "./pages/ChangelogPage";
import { PrivacyApp } from "./pages/PrivacyApp";

import { PrivacyCenterPage } from "./pages/PrivacyCenterPage";
import { ConsentPage } from "./pages/ConsentPage";
import { VaultPage } from "./pages/VaultPage";
import { RedactionPage } from "./pages/RedactionPage";
import { SharingPage } from "./pages/SharingPage";
import { EncryptionPage } from "./pages/EncryptionPage";
import { EventsPage } from "./pages/EventsPage";
import { AuditPage } from "./pages/AuditPage";
import { SettingsPage } from "./pages/SettingsPage";
import { DataMapPage } from "./pages/DataMapPage";
import { SecurityPage } from "./pages/SecurityPage";
import { AccessibilityPage } from "./pages/AccessibilityPage";
import { ResearchPage } from "./pages/ResearchPage";
import { CommunityPage } from "./pages/CommunityPage";
import { SupportPage } from "./pages/SupportPage";

export default function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* Core pages */}
        <Route path="/" element={<LandingPage />} />
        <Route path="/overview" element={<OverviewPage />} />
        <Route path="/privacy" element={<PrivacyPage />} />
        <Route path="/leadership" element={<LeadershipPage />} />
        <Route path="/proof-of-leadership" element={<ProofOfLeadershipPage />} />
        <Route path="/simulator" element={<SimulatorPage />} />
        <Route path="/docs" element={<DocsPage />} />
        <Route path="/examples" element={<ExamplesPage />} />
        <Route path="/roadmap" element={<RoadmapPage />} />
        <Route path="/changelog" element={<ChangelogPage />} />
        <Route path="/dashboard" element={<PrivacyApp />} />

        {/* Privacy features */}
        <Route path="/privacy-center" element={<PrivacyCenterPage />} />
        <Route path="/consent" element={<ConsentPage />} />
        <Route path="/vault" element={<VaultPage />} />
        <Route path="/redaction" element={<RedactionPage />} />
        <Route path="/sharing" element={<SharingPage />} />
        <Route path="/encryption" element={<EncryptionPage />} />
        <Route path="/events" element={<EventsPage />} />
        <Route path="/audit" element={<AuditPage />} />
        <Route path="/settings" element={<SettingsPage />} />
        <Route path="/data-map" element={<DataMapPage />} />
        <Route path="/security" element={<SecurityPage />} />
        <Route path="/accessibility" element={<AccessibilityPage />} />
        <Route path="/research" element={<ResearchPage />} />
        <Route path="/community" element={<CommunityPage />} />
        <Route path="/support" element={<SupportPage />} />

        {/* Fallback */}
        <Route path="*" element={<LandingPage />} />
      </Routes>
    </BrowserRouter>
  );
}
