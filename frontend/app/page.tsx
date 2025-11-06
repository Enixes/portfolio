"use client";

import { useEffect, useState } from "react";

interface Project {
  id: number;
  title: string;
  description: string;
}

export default function Home() {
  const [projects, setProjects] = useState<Project[]>([]);

  useEffect(() => {
    fetch("http://127.0.0.1:8000/projects")
      .then((res) => res.json())
      .then((data) => setProjects(data))
      .catch((err) => console.error(err));
  }, []);

  return (
    <main className="p-6">
      <h1 className="text-3xl font-bold mb-4">🚀 My Projects</h1>
      {projects.length === 0 && <p>Loading...</p>}
      <ul className="space-y-2">
        {projects.map((p) => (
          <li key={p.id} className="border p-3 rounded">
            <h2 className="text-xl font-semibold">{p.title}</h2>
            <p>{p.description}</p>
          </li>
        ))}
      </ul>
    </main>
  );
}