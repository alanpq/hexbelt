import type { BinEntry, BinEntryValue, TreeNode } from '$lib/pkg/rust';

export type VfxProbabilityTableData = {
	keyTimes?: number[];
	keyValues?: number[];
};

export type ValueColor = {
	constantValue?: [number, number, number, number];
	dynamics?: {
		times: number[];
		values: [number, number, number, number][];
		probabilityTables?: VfxProbabilityTableData[];
	};
};

export function assert(cond: unknown, msg?: string): asserts cond {
	if (cond === false) throw msg ?? 'Assertion failed';
}

export const Node = {
	isNamespace: (node?: TreeNode): node is Extract<TreeNode, { kind: 'Namespace' }> =>
		node?.kind === 'Namespace'
} as const;

export const parseVfxProbabilityTableData = (value: BinEntry): VfxProbabilityTableData => {
	assert(value.value.kind === 'PropertyStruct');
	assert(value.value.value.class === '1403439486');

	let keyTimes, keyValues;

	for (const child of value.children) {
		switch (child.name) {
			case 'keyTimes':
				assert(child.value.kind === 'PropertyContainer');
				keyTimes = child.children.map((value) => {
					assert(value.value.kind === 'PropertyJSValue');
					assert(value.value.value.kind === 'F32');
					return value.value.value.value;
				});
				break;
			case 'keyValues':
				assert(child.value.kind === 'PropertyContainer');
				keyValues = child.children.map((value) => {
					assert(value.value.kind === 'PropertyJSValue');
					assert(value.value.value.kind === 'F32');
					return value.value.value.value;
				});
				break;
			default:
				throw new Error(`Invalid prop ${child.name} for VfxProbabilityTableData`);
		}
	}
	return {
		keyTimes,
		keyValues
	};
};

export const parseColor = (value: BinEntry): ValueColor => {
	assert(value.value.kind === 'PropertyEmbedded', 'Color value not PropertyEmbedded');
	assert(
		value.value.value.class === '122655197',
		`Color value not right class (got ${value.value.value.class})`
	);

	let color: ValueColor = { constantValue: undefined, dynamics: undefined };

	for (const child of value.children) {
		switch (child.name) {
			case 'constantValue':
				assert(
					child.value.kind === 'PropertyJSValue',
					`Invalid kind for ColorValue::constantValue - ${child.value.kind}`
				);
				assert(
					child.value.value.kind === 'Vector4',
					`Invalid JSValue kind for ColorValue::constantValue - ${child.value.value.kind}`
				);
				color.constantValue = child.value.value.value;
				assert(color.constantValue);
				break;
			case 'dynamics':
				assert(
					child.value.kind === 'PropertyStruct',
					`Invalid kind for ColorValue::dynamics - ${child.value.kind}`
				);
				assert(
					child.value.value.class === '1128908277',
					'Invalid class for ColorValue::dynamics - ${child.value.value.class}'
				);

				let values, times, probabilityTables;

				for (const prop of child.children) {
					switch (prop.name) {
						case 'values':
							assert(prop.value.kind === 'PropertyContainer');
							values = prop.children.map((child) => {
								assert(child.value.kind === 'PropertyJSValue');
								assert(child.value.value.kind === 'Vector4');
								return child.value.value.value as [number, number, number, number];
							});
							break;
						case 'times':
							assert(prop.value.kind === 'PropertyContainer');
							times = prop.children.map((child) => {
								assert(child.value.kind === 'PropertyJSValue');
								assert(child.value.value.kind === 'F32');
								return child.value.value.value as number;
							});
							break;
						case 'probabilityTables':
							probabilityTables = prop.children.map(parseVfxProbabilityTableData);
							break;
						default:
							throw new Error(`unexpected prop '${prop.name} in VfxAnimatedColorVariableData'`);
					}
				}
				assert(values, "missing prop 'values'");
				assert(times, "missing prop 'times'");

				assert(values.length === times.length, 'color dynamics values/times not same size');

				color.dynamics = {
					values,
					times,
					probabilityTables
				};

				break;
			default:
				break;
		}
	}

	return color;
};
