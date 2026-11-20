import type { Task } from '../types/models';

export class TaskStore {
	tasks = $state<Task[]>([]);

	addTask(task: Task) {
		if (this.tasks.every((t) => t.id !== task.id)) {
			this.tasks.push(task);
		}
	}

	updateTask(updatedTask: Task) {
		const index = this.tasks.findIndex((t) => t.id === updatedTask.id);
		if (index !== -1) {
			this.tasks[index] = updatedTask;
		}
	}

	deleteTask(taskId: string) {
		this.tasks = this.tasks.filter((t) => t.id !== taskId);
	}
}
export const taskStore = new TaskStore();
