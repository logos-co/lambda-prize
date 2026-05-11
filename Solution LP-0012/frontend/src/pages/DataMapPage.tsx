import React from "react";
import { AppShell } from "../components/AppShell";
import { PageHero } from "../components/PageHero";
import { DataMap } from "../components/DataMap";
import { dataCategories } from "../lib/mock-data";

export function DataMapPage() {
  return (
    <AppShell>
      <PageHero
        badge="Data map"
        title="See what is stored, why it exists, and how sensitive it is."
        description="A good privacy UI makes data flows understandable, not hidden. This page surfaces what the system holds and what it does with each category."
        primary={{ to: "/privacy-center", label: "Privacy center" }}
        secondary={{ to: "/security", label: "Security" }}
      />

      <section className="mx-auto max-w-7xl px-6 py-10 pb-24 lg:px-8">
        <DataMap items={dataCategories} />
      </section>
    </AppShell>
  );
}
