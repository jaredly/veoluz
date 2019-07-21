/******/ (function(modules) { // webpackBootstrap
/******/ 	// install a JSONP callback for chunk loading
/******/ 	function webpackJsonpCallback(data) {
/******/ 		var chunkIds = data[0];
/******/ 		var moreModules = data[1];
/******/
/******/
/******/ 		// add "moreModules" to the modules object,
/******/ 		// then flag all "chunkIds" as loaded and fire callback
/******/ 		var moduleId, chunkId, i = 0, resolves = [];
/******/ 		for(;i < chunkIds.length; i++) {
/******/ 			chunkId = chunkIds[i];
/******/ 			if(installedChunks[chunkId]) {
/******/ 				resolves.push(installedChunks[chunkId][0]);
/******/ 			}
/******/ 			installedChunks[chunkId] = 0;
/******/ 		}
/******/ 		for(moduleId in moreModules) {
/******/ 			if(Object.prototype.hasOwnProperty.call(moreModules, moduleId)) {
/******/ 				modules[moduleId] = moreModules[moduleId];
/******/ 			}
/******/ 		}
/******/ 		if(parentJsonpFunction) parentJsonpFunction(data);
/******/
/******/ 		while(resolves.length) {
/******/ 			resolves.shift()();
/******/ 		}
/******/
/******/ 	};
/******/
/******/
/******/ 	// The module cache
/******/ 	var installedModules = {};
/******/
/******/ 	// object to store loaded and loading chunks
/******/ 	// undefined = chunk not loaded, null = chunk preloaded/prefetched
/******/ 	// Promise = chunk loading, 0 = chunk loaded
/******/ 	var installedChunks = {
/******/ 		"main": 0
/******/ 	};
/******/
/******/
/******/
/******/ 	// script path function
/******/ 	function jsonpScriptSrc(chunkId) {
/******/ 		return __webpack_require__.p + "" + chunkId + ".index.js"
/******/ 	}
/******/
/******/ 	// object to store loaded and loading wasm modules
/******/ 	var installedWasmModules = {};
/******/
/******/ 	function promiseResolve() { return Promise.resolve(); }
/******/
/******/ 	var wasmImportObjects = {
/******/ 		"./pkg/zenphoton_bg.wasm": function() {
/******/ 			return {
/******/ 				"./zenphoton": {
/******/ 					"__wbindgen_object_drop_ref": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_object_drop_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_clearTimeout_39fb05679d8a038d": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_clearTimeout_39fb05679d8a038d"](p0i32);
/******/ 					},
/******/ 					"__wbg_deltaY_7fce2b353c09d24a": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_deltaY_7fce2b353c09d24a"](p0i32);
/******/ 					},
/******/ 					"__wbg_preventDefault_1f79b6ade6e8d37b": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_preventDefault_1f79b6ade6e8d37b"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_cb_forget": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_cb_forget"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_cb_drop": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_cb_drop"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_string_new": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_string_new"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_number_new": function(p0f64) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_number_new"](p0f64);
/******/ 					},
/******/ 					"__wbindgen_object_clone_ref": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_object_clone_ref"](p0i32);
/******/ 					},
/******/ 					"__wbg_hash_58c4da3cb0b97894": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_hash_58c4da3cb0b97894"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_static_accessor_location_cfbea43f6234db4f": function() {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_static_accessor_location_cfbea43f6234db4f"]();
/******/ 					},
/******/ 					"__wbindgen_json_parse": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_json_parse"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_json_serialize": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_json_serialize"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_setTimeout_bd577d8c7b83e5c8": function(p0i32,p1f64) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_setTimeout_bd577d8c7b83e5c8"](p0i32,p1f64);
/******/ 					},
/******/ 					"__wbg_new_59cb74e423758ede": function() {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_new_59cb74e423758ede"]();
/******/ 					},
/******/ 					"__wbg_stack_558ba5917b466edd": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_stack_558ba5917b466edd"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_error_4bb6c2a97407129a": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_error_4bb6c2a97407129a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_instanceof_Window": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_instanceof_Window"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_property_CSSStyleDeclaration": function(p0i32,p1i32,p2i32,p3i32,p4i32,p5i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_set_property_CSSStyleDeclaration"](p0i32,p1i32,p2i32,p3i32,p4i32,p5i32);
/******/ 					},
/******/ 					"__widl_instanceof_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_instanceof_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_canvas_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_canvas_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_begin_path_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_begin_path_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_fill_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_fill_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_stroke_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_stroke_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_stroke_style_CanvasRenderingContext2D": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_set_stroke_style_CanvasRenderingContext2D"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_set_fill_style_CanvasRenderingContext2D": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_set_fill_style_CanvasRenderingContext2D"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_put_image_data_CanvasRenderingContext2D": function(p0i32,p1i32,p2f64,p3f64,p4i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_put_image_data_CanvasRenderingContext2D"](p0i32,p1i32,p2f64,p3f64,p4i32);
/******/ 					},
/******/ 					"__widl_f_set_line_dash_CanvasRenderingContext2D": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_set_line_dash_CanvasRenderingContext2D"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_set_line_width_CanvasRenderingContext2D": function(p0i32,p1f64) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_set_line_width_CanvasRenderingContext2D"](p0i32,p1f64);
/******/ 					},
/******/ 					"__widl_f_ellipse_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3f64,p4f64,p5f64,p6f64,p7f64,p8i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_ellipse_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3f64,p4f64,p5f64,p6f64,p7f64,p8i32);
/******/ 					},
/******/ 					"__widl_f_line_to_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_line_to_CanvasRenderingContext2D"](p0i32,p1f64,p2f64);
/******/ 					},
/******/ 					"__widl_f_move_to_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_move_to_CanvasRenderingContext2D"](p0i32,p1f64,p2f64);
/******/ 					},
/******/ 					"__widl_f_clear_rect_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_clear_rect_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__widl_f_fill_rect_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3f64,p4f64) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_fill_rect_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3f64,p4f64);
/******/ 					},
/******/ 					"__widl_f_restore_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_restore_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_save_CanvasRenderingContext2D": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_save_CanvasRenderingContext2D"](p0i32);
/******/ 					},
/******/ 					"__widl_f_scale_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_scale_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3i32);
/******/ 					},
/******/ 					"__widl_f_translate_CanvasRenderingContext2D": function(p0i32,p1f64,p2f64,p3i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_translate_CanvasRenderingContext2D"](p0i32,p1f64,p2f64,p3i32);
/******/ 					},
/******/ 					"__widl_f_x_DOMRect": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_x_DOMRect"](p0i32);
/******/ 					},
/******/ 					"__widl_f_y_DOMRect": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_y_DOMRect"](p0i32);
/******/ 					},
/******/ 					"__widl_f_get_element_by_id_Document": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_get_element_by_id_Document"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_instanceof_Element": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_instanceof_Element"](p0i32);
/******/ 					},
/******/ 					"__widl_f_get_bounding_client_rect_Element": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_get_bounding_client_rect_Element"](p0i32);
/******/ 					},
/******/ 					"__widl_f_prevent_default_Event": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_prevent_default_Event"](p0i32);
/******/ 					},
/******/ 					"__widl_f_target_Event": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_target_Event"](p0i32);
/******/ 					},
/******/ 					"__widl_f_add_event_listener_with_callback_EventTarget": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_add_event_listener_with_callback_EventTarget"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__widl_instanceof_HTMLCanvasElement": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_instanceof_HTMLCanvasElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_get_context_HTMLCanvasElement": function(p0i32,p1i32,p2i32,p3i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_get_context_HTMLCanvasElement"](p0i32,p1i32,p2i32,p3i32);
/******/ 					},
/******/ 					"__widl_f_width_HTMLCanvasElement": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_width_HTMLCanvasElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_width_HTMLCanvasElement": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_set_width_HTMLCanvasElement"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_height_HTMLCanvasElement": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_height_HTMLCanvasElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_height_HTMLCanvasElement": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_set_height_HTMLCanvasElement"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_instanceof_HTMLElement": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_instanceof_HTMLElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_set_inner_text_HTMLElement": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_set_inner_text_HTMLElement"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_style_HTMLElement": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_style_HTMLElement"](p0i32);
/******/ 					},
/******/ 					"__widl_f_new_with_sw_ImageData": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_new_with_sw_ImageData"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_new_with_u8_clamped_array_and_sh_ImageData": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_new_with_u8_clamped_array_and_sh_ImageData"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__widl_f_key_KeyboardEvent": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_key_KeyboardEvent"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_data_MessageEvent": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_data_MessageEvent"](p0i32);
/******/ 					},
/******/ 					"__widl_f_x_MouseEvent": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_x_MouseEvent"](p0i32);
/******/ 					},
/******/ 					"__widl_f_y_MouseEvent": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_y_MouseEvent"](p0i32);
/******/ 					},
/******/ 					"__widl_f_shift_key_MouseEvent": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_shift_key_MouseEvent"](p0i32);
/******/ 					},
/******/ 					"__widl_f_meta_key_MouseEvent": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_meta_key_MouseEvent"](p0i32);
/******/ 					},
/******/ 					"__widl_f_document_Window": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_document_Window"](p0i32);
/******/ 					},
/******/ 					"__widl_f_new_Worker": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_new_Worker"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_post_message_Worker": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_post_message_Worker"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__widl_f_set_onmessage_Worker": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_set_onmessage_Worker"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_log_1_": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_log_1_"](p0i32);
/******/ 					},
/******/ 					"__widl_f_time_with_label_": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_time_with_label_"](p0i32,p1i32);
/******/ 					},
/******/ 					"__widl_f_time_end_with_label_": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__widl_f_time_end_with_label_"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_new_acdbe9c25dc35c37": function() {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_new_acdbe9c25dc35c37"]();
/******/ 					},
/******/ 					"__wbg_push_60b55c9bdc824202": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_push_60b55c9bdc824202"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_newnoargs_a172f39151049128": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_newnoargs_a172f39151049128"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_8a9c8b0a32a202ff": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_call_8a9c8b0a32a202ff"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_call_a8e473c0af4689bc": function(p0i32,p1i32,p2i32,p3i32,p4i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_call_a8e473c0af4689bc"](p0i32,p1i32,p2i32,p3i32,p4i32);
/******/ 					},
/******/ 					"__wbg_buffer_0b401f8e593a961e": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_buffer_0b401f8e593a961e"](p0i32);
/******/ 					},
/******/ 					"__wbg_length_6e1bf15ee11ab799": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_length_6e1bf15ee11ab799"](p0i32);
/******/ 					},
/******/ 					"__wbg_new_6bd7e6ff6abd5d32": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_new_6bd7e6ff6abd5d32"](p0i32);
/******/ 					},
/******/ 					"__wbg_set_5eeb5e2c924b7a1d": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_set_5eeb5e2c924b7a1d"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_newwithbyteoffset_e2825f1ef1f2917c": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_newwithbyteoffset_e2825f1ef1f2917c"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_get_48d637c66043532c": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_get_48d637c66043532c"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_new_3a746f2619705add": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_new_3a746f2619705add"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_call_f54d3a6dadb199ca": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_call_f54d3a6dadb199ca"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_jsval_eq": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_jsval_eq"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_self_ac379e780a0d8b94": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_self_ac379e780a0d8b94"](p0i32);
/******/ 					},
/******/ 					"__wbg_require_6461b1e9a0d7c34a": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_require_6461b1e9a0d7c34a"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbg_crypto_1e4302b85d4f64a2": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_crypto_1e4302b85d4f64a2"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_is_undefined": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_is_undefined"](p0i32);
/******/ 					},
/******/ 					"__wbg_getRandomValues_1b4ba144162a5c9e": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_getRandomValues_1b4ba144162a5c9e"](p0i32);
/******/ 					},
/******/ 					"__wbg_getRandomValues_1ef11e888e5228e9": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_getRandomValues_1ef11e888e5228e9"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbg_randomFillSync_1b52c8482374c55b": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbg_randomFillSync_1b52c8482374c55b"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_number_get": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_number_get"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_debug_string": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_debug_string"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_throw": function(p0i32,p1i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_throw"](p0i32,p1i32);
/******/ 					},
/******/ 					"__wbindgen_rethrow": function(p0i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_rethrow"](p0i32);
/******/ 					},
/******/ 					"__wbindgen_memory": function() {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_memory"]();
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper557": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_closure_wrapper557"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper559": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_closure_wrapper559"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper561": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_closure_wrapper561"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper563": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_closure_wrapper563"](p0i32,p1i32,p2i32);
/******/ 					},
/******/ 					"__wbindgen_closure_wrapper565": function(p0i32,p1i32,p2i32) {
/******/ 						return installedModules["./pkg/zenphoton.js"].exports["__wbindgen_closure_wrapper565"](p0i32,p1i32,p2i32);
/******/ 					}
/******/ 				}
/******/ 			};
/******/ 		},
/******/ 	};
/******/
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/
/******/ 		// Check if module is in cache
/******/ 		if(installedModules[moduleId]) {
/******/ 			return installedModules[moduleId].exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = installedModules[moduleId] = {
/******/ 			i: moduleId,
/******/ 			l: false,
/******/ 			exports: {}
/******/ 		};
/******/
/******/ 		// Execute the module function
/******/ 		modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
/******/
/******/ 		// Flag the module as loaded
/******/ 		module.l = true;
/******/
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/
/******/ 	// This file contains only the entry chunk.
/******/ 	// The chunk loading function for additional chunks
/******/ 	__webpack_require__.e = function requireEnsure(chunkId) {
/******/ 		var promises = [];
/******/
/******/
/******/ 		// JSONP chunk loading for javascript
/******/
/******/ 		var installedChunkData = installedChunks[chunkId];
/******/ 		if(installedChunkData !== 0) { // 0 means "already installed".
/******/
/******/ 			// a Promise means "currently loading".
/******/ 			if(installedChunkData) {
/******/ 				promises.push(installedChunkData[2]);
/******/ 			} else {
/******/ 				// setup Promise in chunk cache
/******/ 				var promise = new Promise(function(resolve, reject) {
/******/ 					installedChunkData = installedChunks[chunkId] = [resolve, reject];
/******/ 				});
/******/ 				promises.push(installedChunkData[2] = promise);
/******/
/******/ 				// start chunk loading
/******/ 				var script = document.createElement('script');
/******/ 				var onScriptComplete;
/******/
/******/ 				script.charset = 'utf-8';
/******/ 				script.timeout = 120;
/******/ 				if (__webpack_require__.nc) {
/******/ 					script.setAttribute("nonce", __webpack_require__.nc);
/******/ 				}
/******/ 				script.src = jsonpScriptSrc(chunkId);
/******/
/******/ 				// create error before stack unwound to get useful stacktrace later
/******/ 				var error = new Error();
/******/ 				onScriptComplete = function (event) {
/******/ 					// avoid mem leaks in IE.
/******/ 					script.onerror = script.onload = null;
/******/ 					clearTimeout(timeout);
/******/ 					var chunk = installedChunks[chunkId];
/******/ 					if(chunk !== 0) {
/******/ 						if(chunk) {
/******/ 							var errorType = event && (event.type === 'load' ? 'missing' : event.type);
/******/ 							var realSrc = event && event.target && event.target.src;
/******/ 							error.message = 'Loading chunk ' + chunkId + ' failed.\n(' + errorType + ': ' + realSrc + ')';
/******/ 							error.type = errorType;
/******/ 							error.request = realSrc;
/******/ 							chunk[1](error);
/******/ 						}
/******/ 						installedChunks[chunkId] = undefined;
/******/ 					}
/******/ 				};
/******/ 				var timeout = setTimeout(function(){
/******/ 					onScriptComplete({ type: 'timeout', target: script });
/******/ 				}, 120000);
/******/ 				script.onerror = script.onload = onScriptComplete;
/******/ 				document.head.appendChild(script);
/******/ 			}
/******/ 		}
/******/
/******/ 		// Fetch + compile chunk loading for webassembly
/******/
/******/ 		var wasmModules = {"3":["./pkg/zenphoton_bg.wasm"]}[chunkId] || [];
/******/
/******/ 		wasmModules.forEach(function(wasmModuleId) {
/******/ 			var installedWasmModuleData = installedWasmModules[wasmModuleId];
/******/
/******/ 			// a Promise means "currently loading" or "already loaded".
/******/ 			if(installedWasmModuleData)
/******/ 				promises.push(installedWasmModuleData);
/******/ 			else {
/******/ 				var importObject = wasmImportObjects[wasmModuleId]();
/******/ 				var req = fetch(__webpack_require__.p + "" + {"./pkg/zenphoton_bg.wasm":"639d1d39026502336027"}[wasmModuleId] + ".module.wasm");
/******/ 				var promise;
/******/ 				if(importObject instanceof Promise && typeof WebAssembly.compileStreaming === 'function') {
/******/ 					promise = Promise.all([WebAssembly.compileStreaming(req), importObject]).then(function(items) {
/******/ 						return WebAssembly.instantiate(items[0], items[1]);
/******/ 					});
/******/ 				} else if(typeof WebAssembly.instantiateStreaming === 'function') {
/******/ 					promise = WebAssembly.instantiateStreaming(req, importObject);
/******/ 				} else {
/******/ 					var bytesPromise = req.then(function(x) { return x.arrayBuffer(); });
/******/ 					promise = bytesPromise.then(function(bytes) {
/******/ 						return WebAssembly.instantiate(bytes, importObject);
/******/ 					});
/******/ 				}
/******/ 				promises.push(installedWasmModules[wasmModuleId] = promise.then(function(res) {
/******/ 					return __webpack_require__.w[wasmModuleId] = (res.instance || res).exports;
/******/ 				}));
/******/ 			}
/******/ 		});
/******/ 		return Promise.all(promises);
/******/ 	};
/******/
/******/ 	// expose the modules object (__webpack_modules__)
/******/ 	__webpack_require__.m = modules;
/******/
/******/ 	// expose the module cache
/******/ 	__webpack_require__.c = installedModules;
/******/
/******/ 	// define getter function for harmony exports
/******/ 	__webpack_require__.d = function(exports, name, getter) {
/******/ 		if(!__webpack_require__.o(exports, name)) {
/******/ 			Object.defineProperty(exports, name, { enumerable: true, get: getter });
/******/ 		}
/******/ 	};
/******/
/******/ 	// define __esModule on exports
/******/ 	__webpack_require__.r = function(exports) {
/******/ 		if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 			Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 		}
/******/ 		Object.defineProperty(exports, '__esModule', { value: true });
/******/ 	};
/******/
/******/ 	// create a fake namespace object
/******/ 	// mode & 1: value is a module id, require it
/******/ 	// mode & 2: merge all properties of value into the ns
/******/ 	// mode & 4: return value when already ns object
/******/ 	// mode & 8|1: behave like require
/******/ 	__webpack_require__.t = function(value, mode) {
/******/ 		if(mode & 1) value = __webpack_require__(value);
/******/ 		if(mode & 8) return value;
/******/ 		if((mode & 4) && typeof value === 'object' && value && value.__esModule) return value;
/******/ 		var ns = Object.create(null);
/******/ 		__webpack_require__.r(ns);
/******/ 		Object.defineProperty(ns, 'default', { enumerable: true, value: value });
/******/ 		if(mode & 2 && typeof value != 'string') for(var key in value) __webpack_require__.d(ns, key, function(key) { return value[key]; }.bind(null, key));
/******/ 		return ns;
/******/ 	};
/******/
/******/ 	// getDefaultExport function for compatibility with non-harmony modules
/******/ 	__webpack_require__.n = function(module) {
/******/ 		var getter = module && module.__esModule ?
/******/ 			function getDefault() { return module['default']; } :
/******/ 			function getModuleExports() { return module; };
/******/ 		__webpack_require__.d(getter, 'a', getter);
/******/ 		return getter;
/******/ 	};
/******/
/******/ 	// Object.prototype.hasOwnProperty.call
/******/ 	__webpack_require__.o = function(object, property) { return Object.prototype.hasOwnProperty.call(object, property); };
/******/
/******/ 	// __webpack_public_path__
/******/ 	__webpack_require__.p = "";
/******/
/******/ 	// on error function for async loading
/******/ 	__webpack_require__.oe = function(err) { console.error(err); throw err; };
/******/
/******/ 	// object with all WebAssembly.instance exports
/******/ 	__webpack_require__.w = {};
/******/
/******/ 	var jsonpArray = window["webpackJsonp"] = window["webpackJsonp"] || [];
/******/ 	var oldJsonpFunction = jsonpArray.push.bind(jsonpArray);
/******/ 	jsonpArray.push = webpackJsonpCallback;
/******/ 	jsonpArray = jsonpArray.slice();
/******/ 	for(var i = 0; i < jsonpArray.length; i++) webpackJsonpCallback(jsonpArray[i]);
/******/ 	var parentJsonpFunction = oldJsonpFunction;
/******/
/******/
/******/ 	// Load entry module and return exports
/******/ 	return __webpack_require__(__webpack_require__.s = "./index.js");
/******/ })
/************************************************************************/
/******/ ({

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no static exports found */
/***/ (function(module, exports, __webpack_require__) {

eval("\nPromise.all(/*! import() */[__webpack_require__.e(0), __webpack_require__.e(1)]).then(__webpack_require__.t.bind(null, /*! ./ui/App.bs.js */ \"./ui/App.bs.js\", 7));\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ })

/******/ });