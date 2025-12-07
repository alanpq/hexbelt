import type { Item } from './pkg/rust';

export type PreviewKind = 'texture' | 'mesh';

const fromSets = (m: { [K in PreviewKind]: Set<string> }) => {
	const out: { [ext: string]: PreviewKind } = {};
	for (const [k, v] of Object.entries(m)) {
		for (const i of v) {
			out[i] = k as PreviewKind;
		}
	}
	return out;
};

export const previewableExtensions = fromSets({
	texture: new Set(['dds', 'jpg', 'png', 'tex', 'svg']),
	mesh: new Set(['sco', 'scb'])
});
export const getPreviewKind = (item: Item): PreviewKind | undefined => {
	const ext = item.name.split('.').at(-1);
	if (!ext) return undefined;

	return previewableExtensions[ext];
};
