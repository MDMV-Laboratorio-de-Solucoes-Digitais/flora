import { describe, it, expect } from 'vitest';
import { tasksEndpoint } from '../../src/lib/components/tasks/tasks-query';

describe('tasksEndpoint', () => {
	it('keeps the legacy organization-wide endpoint when no workspace is selected', () => {
		expect(tasksEndpoint(null)).toBe('/api/v1/tasks');
		expect(tasksEndpoint('')).toBe('/api/v1/tasks');
	});

	it('scopes the request to the selected workspace', () => {
		expect(tasksEndpoint('01a20043-d9d8-7ce0-a2f1-eeafee124ef6')).toBe(
			'/api/v1/tasks?workspace_id=01a20043-d9d8-7ce0-a2f1-eeafee124ef6'
		);
	});

	it('encodes characters that could alter the query string', () => {
		expect(tasksEndpoint('ws 1&admin=true')).toBe(
			'/api/v1/tasks?workspace_id=ws%201%26admin%3Dtrue'
		);
	});
});
