import { toast } from "vue-sonner";

import { useModeratorNotes } from "~/scripts/pages/moderation";

export function useReviewNotes(slug: string) {
	const { notes: moderatorNotes, submitting: noteSubmitting, load: loadNotes, add: addModeratorNote } = useModeratorNotes(slug);

	const newNote = ref("");

	async function submitNote() {
		const body = newNote.value.trim();
		if (!body) return;
		const ok = await addModeratorNote(body);
		if (ok) {
			newNote.value = "";
			toast.success("Note added.");
		} else {
			toast.error("Could not save your note.");
		}
	}

	return { moderatorNotes, noteSubmitting, newNote, submitNote, loadNotes };
}
