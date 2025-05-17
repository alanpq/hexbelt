import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export const normToRgba = (norm: [number, number, number, number]) => {
	return [norm[0] * 255, norm[1] * 255, norm[2] * 255, norm[3]];
};
