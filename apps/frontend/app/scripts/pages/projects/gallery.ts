import { useApi, apiErrorMessage } from "~/scripts/api";
import type { GalleryImage } from "./types";

export function useGallery(slug: string) {
	const config = useRuntimeConfig();
	const api = useApi();

	const images = ref<GalleryImage[]>([]);

	async function load() {
		try {
			const data = await api<{ images: GalleryImage[] }>(`/projects/${slug}/gallery`);
			images.value = data.images.map((image) => ({
				...image,
				url: `${config.public.apiBase}${image.url}`,
			}));
		} catch {
			images.value = [];
		}
	}

	async function remove(id: string): Promise<boolean> {
		try {
			await api(`/projects/${slug}/gallery/${id}`, {
				method: "DELETE",
			});
			await load();
			return true;
		} catch {
			return false;
		}
	}

	return { images, load, remove };
}

export function useUploadGalleryForm(slug: string) {
	const api = useApi();

	const caption = ref("");
	const image = ref<File | null>(null);
	const error = ref("");
	const pending = ref(false);

	function onFileChange(event: Event) {
		const input = event.target as HTMLInputElement;
		image.value = input.files?.[0] ?? null;
	}

	async function submit(): Promise<boolean> {
		error.value = "";

		if (!image.value) {
			error.value = "Please choose an image to upload.";
			return false;
		}

		const body = new FormData();
		body.append("caption", caption.value);
		body.append("image", image.value);

		pending.value = true;
		try {
			await api(`/projects/${slug}/gallery`, {
				method: "POST",
				body,
			});
			caption.value = "";
			image.value = null;
			return true;
		} catch (err) {
			error.value = apiErrorMessage(err, {
				fallback: "Could not upload the image. Please try again.",
				status: { 401: "Please sign in to upload an image." },
			});
			return false;
		} finally {
			pending.value = false;
		}
	}

	return { caption, image, error, pending, onFileChange, submit };
}
