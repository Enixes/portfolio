// app/page.tsx
export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center justify-center bg-gray-50">
      <h1 className="text-4xl font-bold text-gray-800 mb-4">
        Welcome to My Portfolio
      </h1>
      <p className="text-gray-600">
        Built with <span className="font-semibold text-blue-600">Next.js</span> +{" "}
        <span className="font-semibold text-orange-500">Rust (Axum)</span>
      </p>
    </main>
  );
}
