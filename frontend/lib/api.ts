const API_URL = process.env.NEXT_PUBLIC_API_URL;

export async function getProjects() {
    const res = await fetch(`${API_URL}/projects`, {
        next: {
            revalidate: 10,
        },
    });
    if (!res.ok) {
        throw new Error("Failed to fetch projects from backend")
    }
    return res.json();
}