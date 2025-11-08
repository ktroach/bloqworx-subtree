/* tslint:disable */
/* eslint-disable */
export function get_memory_size_mb(): number;
export function get_instance_ptr(instance: SubTree): number;
export class Property {
  free(): void;
  constructor(name: string, value: string, class_name?: string | null, id?: string | null);
  set_value(val: string): void;
  setClassName(class_name: string): void;
  readonly id: string;
  readonly name: string;
  readonly value: string;
  readonly className: string;
}
export class SubTree {
  free(): void;
  constructor();
  add_node(value: string, parent_id?: string | null, node_id?: string | null): SubTreeNode;
  /**
   * Set a property on a node without cloning the node
   */
  set_node_property(node_id: string, name: string, value: any): boolean;
  set_node_property_with_class_name(node_id: string, name: string, value: any, class_name?: string | null): boolean;
  /**
   * Get a property from a node without cloning the node
   */
  get_node_property(node_id: string, name: string): any;
  /**
   * Get property ID by index without cloning the node
   */
  get_node_property_id(node_id: string, idx: number): string;
  /**
   * Get property count for a node without cloning the node
   */
  get_node_property_count(node_id: string): number;
  /**
   * Get property name by index without cloning the node
   */
  get_node_property_name(node_id: string, idx: number): string;
  /**
   * Get property value by index without cloning the node
   */
  get_node_property_value(node_id: string, idx: number): any;
  /**
   * Get property className by index without cloning the node
   */
  get_node_property_class_name(node_id: string, idx: number): string;
  /**
   * Get all child IDs for a node without cloning
   */
  get_node_child_ids(node_id: string): Array<any>;
  /**
   * Check if a node exists
   */
  node_exists(node_id: string): boolean;
  /**
   * Get node value without cloning
   */
  get_node_value(node_id: string): string;
  /**
   * Set size_state (JS object) on a node without cloning
   */
  set_node_size_state(node_id: string, state: any): boolean;
  /**
   * Get size_state (JS object) from a node without cloning
   */
  get_node_size_state(node_id: string): any;
  /**
   * Set tailwind_size string on a node without cloning
   */
  set_node_tailwind_size(node_id: string, tailwind: string): boolean;
  /**
   * Get tailwind_size string from a node without cloning
   */
  get_node_tailwind_size(node_id: string): any;
  /**
   * ⚠️ LEGACY: Returns a clone - properties might not persist
   */
  find_node(id: string): SubTreeNode | undefined;
  /**
   * ⚠️ LEGACY: Returns a clone - properties might not persist  
   */
  get_root(): SubTreeNode | undefined;
  remove_node(id: string): void;
  is_empty(): boolean;
  get_node_count(): number;
  get_height(): number;
  is_balanced(): boolean;
  clear_subtree_except_root(): void;
}
export class SubTreeNode {
  free(): void;
  constructor(value: string, id?: string | null);
  add_child(child: SubTreeNode): void;
  get_children(): Array<any>;
  set_property(name: string, value: any): void;
  set_property_with_class_name(name: string, value: any, class_name?: string | null): void;
  get_property(name: string): any;
  get_property_count(): number;
  get_property_id(idx: number): string;
  get_property_name(idx: number): string;
  get_property_value(idx: number): any;
  get_property_class_name(idx: number): string;
  has_property(name: string): boolean;
  set_size_state(state: any): void;
  get_size_state(): any;
  set_tailwind_size(tailwind: string): void;
  get_tailwind_size(): any;
  set_style_state(state: any): void;
  get_style_state(): any;
  set_tailwind_style(tailwind: string): void;
  get_tailwind_style(): any;
  readonly id: string;
  readonly value: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_memory_size_mb: () => number;
  readonly get_instance_ptr: (a: number) => number;
  readonly __wbg_property_free: (a: number, b: number) => void;
  readonly property_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly property_id: (a: number) => [number, number];
  readonly property_name: (a: number) => [number, number];
  readonly property_value: (a: number) => [number, number];
  readonly property_className: (a: number) => [number, number];
  readonly property_set_value: (a: number, b: number, c: number) => void;
  readonly property_setClassName: (a: number, b: number, c: number) => void;
  readonly __wbg_subtreenode_free: (a: number, b: number) => void;
  readonly subtreenode_new: (a: number, b: number, c: number, d: number) => number;
  readonly subtreenode_id: (a: number) => [number, number];
  readonly subtreenode_value: (a: number) => [number, number];
  readonly subtreenode_add_child: (a: number, b: number) => void;
  readonly subtreenode_get_children: (a: number) => any;
  readonly subtreenode_set_property: (a: number, b: number, c: number, d: any) => void;
  readonly subtreenode_set_property_with_class_name: (a: number, b: number, c: number, d: any, e: number, f: number) => void;
  readonly subtreenode_get_property: (a: number, b: number, c: number) => any;
  readonly subtreenode_get_property_count: (a: number) => number;
  readonly subtreenode_get_property_id: (a: number, b: number) => [number, number];
  readonly subtreenode_get_property_name: (a: number, b: number) => [number, number];
  readonly subtreenode_get_property_value: (a: number, b: number) => any;
  readonly subtreenode_get_property_class_name: (a: number, b: number) => [number, number];
  readonly subtreenode_has_property: (a: number, b: number, c: number) => number;
  readonly subtreenode_set_size_state: (a: number, b: any) => void;
  readonly subtreenode_get_size_state: (a: number) => any;
  readonly subtreenode_set_tailwind_size: (a: number, b: number, c: number) => void;
  readonly subtreenode_get_tailwind_size: (a: number) => any;
  readonly subtreenode_set_style_state: (a: number, b: any) => void;
  readonly subtreenode_get_style_state: (a: number) => any;
  readonly subtreenode_set_tailwind_style: (a: number, b: number, c: number) => void;
  readonly subtreenode_get_tailwind_style: (a: number) => any;
  readonly __wbg_subtree_free: (a: number, b: number) => void;
  readonly subtree_new: () => number;
  readonly subtree_add_node: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly subtree_set_node_property: (a: number, b: number, c: number, d: number, e: number, f: any) => number;
  readonly subtree_set_node_property_with_class_name: (a: number, b: number, c: number, d: number, e: number, f: any, g: number, h: number) => number;
  readonly subtree_get_node_property: (a: number, b: number, c: number, d: number, e: number) => any;
  readonly subtree_get_node_property_id: (a: number, b: number, c: number, d: number) => [number, number];
  readonly subtree_get_node_property_count: (a: number, b: number, c: number) => number;
  readonly subtree_get_node_property_name: (a: number, b: number, c: number, d: number) => [number, number];
  readonly subtree_get_node_property_value: (a: number, b: number, c: number, d: number) => any;
  readonly subtree_get_node_property_class_name: (a: number, b: number, c: number, d: number) => [number, number];
  readonly subtree_get_node_child_ids: (a: number, b: number, c: number) => any;
  readonly subtree_node_exists: (a: number, b: number, c: number) => number;
  readonly subtree_get_node_value: (a: number, b: number, c: number) => [number, number];
  readonly subtree_set_node_size_state: (a: number, b: number, c: number, d: any) => number;
  readonly subtree_get_node_size_state: (a: number, b: number, c: number) => any;
  readonly subtree_set_node_tailwind_size: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly subtree_get_node_tailwind_size: (a: number, b: number, c: number) => any;
  readonly subtree_find_node: (a: number, b: number, c: number) => number;
  readonly subtree_get_root: (a: number) => number;
  readonly subtree_remove_node: (a: number, b: number, c: number) => void;
  readonly subtree_is_empty: (a: number) => number;
  readonly subtree_get_node_count: (a: number) => number;
  readonly subtree_get_height: (a: number) => number;
  readonly subtree_is_balanced: (a: number) => number;
  readonly subtree_clear_subtree_except_root: (a: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
