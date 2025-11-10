import { getProjects } from "@/lib/api";

type Project = {
  id: number;
  title: string;
  description: string;
};

export default async function ProjectsPage() {
  const projects: Project[] = await getProjects();

  return (
    <main className="min-h-screen bg-gray-50 p-10">
      <h1 className="text-4xl font-bold mb-8 text-center text-gray-800">
        My Projects
      </h1>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {projects.map((project) => (
          <div
            key={project.id}
            className="bg-white shadow-lg rounded-xl p-6 hover:shadow-2xl transition"
          >
            <h2 className="text-2xl font-semibold text-gray-700 mb-2">
              {project.title}
            </h2>
            <p className="text-gray-600">{project.description}</p>
          </div>
        ))}
      </div>
    </main>
  );
}
