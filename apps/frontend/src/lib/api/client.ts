import { authState } from '../state/AuthState.svelte';
import type { SearchResponse } from '../types/models';

export const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'; // Default local Rust backend URL

export type ApiErrorBody = {
	error?: string;
	message?: string;
	field?: string;
	[key: string]: unknown;
};

export class ApiError extends Error {
	readonly status: number;
	readonly statusText: string;
	readonly url: string;
	readonly code?: string;
	readonly field?: string;
	readonly body: ApiErrorBody | string | null;

	constructor(
		status: number,
		statusText: string,
		url: string,
		message: string,
		body: ApiErrorBody | string | null,
		code?: string,
		field?: string
	) {
		super(message);
		this.name = 'ApiError';
		this.status = status;
		this.statusText = statusText;
		this.url = url;
		this.code = code;
		this.field = field;
		this.body = body;
	}
}

async function parseResponseBody(response: Response): Promise<ApiErrorBody | string | null> {
	if (response.status === 204 || response.status === 205) return null;
	const text = await response.text();
	if (!text) return null;
	try {
		return JSON.parse(text) as ApiErrorBody;
	} catch {
		return text;
	}
}

function messageFromBody(body: ApiErrorBody | string | null, fallback: string): string {
	if (!body) return fallback;
	if (typeof body === 'string') return body;
	if (typeof body.message === 'string' && body.message) return body.message;
	if (typeof body.error === 'string' && body.error) return body.error;
	return fallback;
}

function apiErrorFromResponse(
	response: Response,
	url: string,
	body: ApiErrorBody | string | null,
	fallbackCode?: string
): ApiError {
	const code =
		(body && typeof body === 'object' ? (body.error as string | undefined) : undefined) ??
		fallbackCode;
	const field = body && typeof body === 'object' ? (body.field as string | undefined) : undefined;
	const message = messageFromBody(body, `API Error ${response.status}`);
	return new ApiError(response.status, response.statusText, url, message, body, code, field);
}

export const ApiClient = {
	async request<T>(endpoint: string, options: RequestInit = {}): Promise<T> {
		const url = `${API_BASE_URL}${endpoint.startsWith('/') ? endpoint : `/${endpoint}`}`;
		const headers = new Headers(options.headers || {});

		if (authState.token && !headers.has('Authorization')) {
			headers.set('Authorization', `Bearer ${authState.token}`);
		}

		const isFormData = typeof FormData !== 'undefined' && options.body instanceof FormData;
		if (
			!isFormData &&
			!headers.has('Content-Type') &&
			options.body != null &&
			!(options.body instanceof Blob)
		) {
			headers.set('Content-Type', 'application/json');
		}

		const response = await fetch(url, { ...options, headers });
		const body = await parseResponseBody(response);

		if (response.status === 401) {
			authState.lockSession();
			throw apiErrorFromResponse(response, url, body, 'unauthorized');
		}

		if (!response.ok) {
			throw apiErrorFromResponse(response, url, body);
		}

		if (response.status === 204) {
			return {} as T;
		}

		return (body as T) ?? ({} as T);
	},

	/** GET /api/v1/search?q=… — authoritative Postgres FTS. */
	async search(parameters: { q: string; type?: string; limit?: number }): Promise<SearchResponse> {
		const query = new URLSearchParams({ q: parameters.q });
		if (parameters.type) query.set('type', parameters.type);
		if (parameters.limit !== undefined) query.set('limit', String(parameters.limit));
		return ApiClient.request<SearchResponse>(`/api/v1/search?${query.toString()}`);
	}
};
