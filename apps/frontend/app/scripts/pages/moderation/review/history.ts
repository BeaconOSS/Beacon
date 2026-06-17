const HISTORY_NOTE_CLAMP = 160;

export function useReviewHistory() {
	const expandedHistory = ref<Set<number>>(new Set());

	function toggleHistory(idx: number) {
		const next = new Set(expandedHistory.value);
		if (next.has(idx)) next.delete(idx);
		else next.add(idx);
		expandedHistory.value = next;
	}

	return { expandedHistory, toggleHistory, historyNoteClamp: HISTORY_NOTE_CLAMP };
}
