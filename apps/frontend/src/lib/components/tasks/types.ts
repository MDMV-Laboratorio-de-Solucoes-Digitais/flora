/** Minimal task shape consumed by the Kanban board (snake_case from the API). */
export type BoardTask = {
	id: string;
	title: string;
	description?: string | null;
	status: string;
};
