import type { Item } from './pkg/rust';

export const previewableExtensions = new Set(['dds', 'jpg', 'png', 'tex', 'svg']);
export const isPreviewable = (item: Item): boolean => {
	const ext = item.name.split('.').at(-1);
	if (!ext) return false;

	return previewableExtensions.has(ext);
};
