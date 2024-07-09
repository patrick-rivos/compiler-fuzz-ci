use arbitrary::Arbitrary;
use std::fmt;
use struct_iterable::Iterable;

use crate::{GhostOpt, ToggleOpt};

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Arbitrary, Debug, Iterable, Clone)]
pub struct LlvmFlags {
    faapcs_bitfield_width: ToggleOpt,
    faccess_control: ToggleOpt,
    faddrsig: ToggleOpt,
    faligned_allocation: ToggleOpt,
    fallow_editor_placeholders: ToggleOpt,
    fansi_escape_codes: GhostOpt,
    fapinotes: ToggleOpt,
    fapinotes_modules: ToggleOpt,
    //     fapple_kext: GhostOpt, // ??? - Revisit
    fapple_link_rtlib: GhostOpt,
    fapple_pragma_pack: ToggleOpt,
    fapplication_extension: ToggleOpt,
    fapprox_func: ToggleOpt,
    fassume_nothrow_exception_dtor: ToggleOpt,
    fassume_sane_operator_new: ToggleOpt,
    fassume_unique_vtables: ToggleOpt,
    fassumptions: ToggleOpt,
    fasync_exceptions: ToggleOpt,
    //     fauto_import: GhostOpt, // Unsupported
    fautolink: ToggleOpt,
    //     fbasic_block_address_map: GhostOpt, // Unsupported
    fblocks: ToggleOpt,
    fborland_extensions: ToggleOpt,
    fbuiltin: ToggleOpt,
    fbuiltin_module_map: ToggleOpt,
    //     fcf_protection: GhostOpt, // Unsupported - Weird error msg
    fchar8_t: ToggleOpt,
    fcheck_new: ToggleOpt,
    fcolor_diagnostics: ToggleOpt,
    fcommon: ToggleOpt,
    fcomplete_member_pointers: ToggleOpt,
    fconstant_cfstrings: ToggleOpt,
    fconvergent_functions: ToggleOpt,
    fcoro_aligned_allocation: ToggleOpt,
    fcoroutines: ToggleOpt,
    fcoverage_mapping: ToggleOpt,
    fcoverage_mcdc: ToggleOpt,
    fcrash_diagnostics: ToggleOpt,
    fcs_profile_generate: GhostOpt,
    fcuda_short_ptr: ToggleOpt,
    fcx_fortran_rules: ToggleOpt,
    fcx_limited_range: ToggleOpt,
    fcxx_exceptions: ToggleOpt,
    fcxx_modules: ToggleOpt,
    fdata_sections: ToggleOpt,
    fdebug_info_for_profiling: ToggleOpt,
    fdebug_macro: ToggleOpt,
    fdebug_ranges_base_address: ToggleOpt,
    fdebug_types_section: ToggleOpt,
    fdeclspec: ToggleOpt,
    fdefine_target_os_macros: ToggleOpt,
    fdelayed_template_parsing: ToggleOpt,
    fdelete_null_pointer_checks: ToggleOpt,
    fdiagnostics_absolute_paths: GhostOpt,
    fdiagnostics_fixit_info: ToggleOpt,
    fdiagnostics_parseable_fixits: GhostOpt,
    fdiagnostics_print_source_range_info: GhostOpt,
    fdiagnostics_show_hotness: ToggleOpt,
    fdiagnostics_show_line_numbers: ToggleOpt,
    fdiagnostics_show_note_include_stack: ToggleOpt,
    fdiagnostics_show_option: ToggleOpt,
    fdiagnostics_show_template_tree: GhostOpt,
    fdirect_access_external_data: ToggleOpt,
    fdiscard_value_names: ToggleOpt,
    fdollars_in_identifiers: ToggleOpt,
    fdriver_only: GhostOpt,
    fdwarf_exceptions: GhostOpt,
    felide_constructors: ToggleOpt,
    fno_elide_type: GhostOpt,
    feliminate_unused_debug_types: ToggleOpt,
    fembed_bitcode: GhostOpt,
    fembed_bitcode_marker: GhostOpt,
    femit_all_decls: GhostOpt,
    femit_compact_unwind_non_canonical: ToggleOpt,
    femulated_tls: ToggleOpt,
    fenable_matrix: GhostOpt,
    fexceptions: ToggleOpt,
    fexperimental_library: ToggleOpt,
    //     fexperimental_new_constant_interpreter: GhostOpt, // https://github.com/llvm/llvm-project/issues/88018#issuecomment-2043346140
    fexperimental_strict_floating_point: GhostOpt,
    ffast_math: ToggleOpt,
    ffat_lto_objects: ToggleOpt,
    ffile_reproducible: ToggleOpt,
    ffine_grained_bitfield_accesses: ToggleOpt,
    ffinite_loops: ToggleOpt,
    ffinite_math_only: ToggleOpt,
    ffixed_point: ToggleOpt,
    //     ffixed_x1: GhostOpt, // error: Return address register required, but has been reserved.
    //     ffixed_x10: GhostOpt, // error: Argument register required, but has been reserved.
    //     ffixed_x11: GhostOpt, // error: Argument register required, but has been reserved.
    //     ffixed_x12: GhostOpt, // error: Argument register required, but has been reserved.
    //     ffixed_x13: GhostOpt, // error: Argument register required, but has been reserved.
    //     ffixed_x14: GhostOpt, // error: Argument register required, but has been reserved.
    //     ffixed_x15: GhostOpt, // error: Argument register required, but has been reserved.
    //     ffixed_x16: GhostOpt, // error: Argument register required, but has been reserved.
    //     ffixed_x17: GhostOpt, // error: Argument register required, but has been reserved.
    ffixed_x18: GhostOpt,
    ffixed_x19: GhostOpt,
    //     ffixed_x2: GhostOpt, // error: Stack pointer required, but has been reserved.
    ffixed_x20: GhostOpt,
    ffixed_x21: GhostOpt,
    ffixed_x22: GhostOpt,
    ffixed_x23: GhostOpt,
    ffixed_x24: GhostOpt,
    ffixed_x25: GhostOpt,
    ffixed_x26: GhostOpt,
    ffixed_x27: GhostOpt,
    ffixed_x28: GhostOpt,
    ffixed_x29: GhostOpt,
    ffixed_x3: GhostOpt,
    ffixed_x30: GhostOpt,
    ffixed_x31: GhostOpt,
    ffixed_x4: GhostOpt,
    ffixed_x5: GhostOpt,
    ffixed_x6: GhostOpt,
    ffixed_x7: GhostOpt,
    //     ffixed_x8: GhostOpt, // error: Frame pointer required, but has been reserved.
    ffixed_x9: GhostOpt,
    fforce_check_cxx20_modules_input_files: GhostOpt,
    fforce_dwarf_frame: ToggleOpt,
    fforce_emit_vtables: ToggleOpt,
    fforce_enable_int128: ToggleOpt,
    ffreestanding: GhostOpt,
    ffunction_sections: ToggleOpt,
    fglobal_isel: ToggleOpt,
    fgnu_inline_asm: ToggleOpt,
    fgnu_keywords: ToggleOpt,
    fgnu_runtime: GhostOpt,
    fgnu89_inline: ToggleOpt,
    fgpu_allow_device_init: ToggleOpt,
    fgpu_approx_transcendentals: ToggleOpt,
    fgpu_defer_diag: ToggleOpt,
    fgpu_exclude_wrong_side_overloads: ToggleOpt,
    fgpu_flush_denormals_to_zero: ToggleOpt,
    fgpu_rdc: ToggleOpt,
    fgpu_sanitize: ToggleOpt,
    //     fhip_emit_relocatable: ToggleOpt, // Required --cuda-device-only
    fhip_fp32_correctly_rounded_divide_sqrt: ToggleOpt,
    fhip_kernel_arg_name: ToggleOpt,
    fhip_new_launch_api: ToggleOpt,
    fhonor_infinities: ToggleOpt,
    fhonor_nans: ToggleOpt,
    fignore_exceptions: GhostOpt,
    fimplicit_module_maps: ToggleOpt,
    fincremental_extensions: GhostOpt,
    finline_functions: ToggleOpt,
    finline_hint_functions: GhostOpt,
    //     finstrument_function_entry_bare: GhostOpt, // ??? - Revisit
    finstrument_functions: GhostOpt,
    finstrument_functions_after_inlining: GhostOpt,
    pub fintegrated_as: ToggleOpt,
    fintegrated_cc1: ToggleOpt,
    fintegrated_objemitter: GhostOpt,
    fjmc: ToggleOpt,
    fjump_tables: ToggleOpt,
    fkeep_persistent_storage_variables: ToggleOpt,
    fkeep_static_consts: ToggleOpt,
    fkeep_system_includes: ToggleOpt,
    fno_knr_functions: GhostOpt,
    flto: ToggleOpt,
    fmath_errno: ToggleOpt,
    //     fmemory_profile: ToggleOpt, // riscv64-unknown-linux-gnu/libclang_rt.memprof.a: No such file or directory
    fmerge_all_constants: ToggleOpt,
    fminimize_whitespace: ToggleOpt,
    fmodule_header: GhostOpt,
    fmodule_output: GhostOpt,
    fmodules: ToggleOpt,
    fmodules_decluse: ToggleOpt,
    fmodules_disable_diagnostic_validation: GhostOpt,
    fmodules_search_all: ToggleOpt,
    //     fmodules_strict_decluse: GhostOpt, // error: module _Builtin_stdint does not depend on a module exporting 'stdint.h'
    fno_modules_validate_input_files_content: GhostOpt,
    //     fno_modules_validate_once_per_build_session: GhostOpt, // Unknown Argument
    fmodules_validate_system_headers: ToggleOpt,
    fno_modules_validate_textual_header_includes: GhostOpt,
    fms_compatibility: ToggleOpt,
    fms_extensions: ToggleOpt,
    fms_hotpatch: GhostOpt,
    fms_volatile: ToggleOpt,
    fnew_infallible: ToggleOpt,
    fobjc_arc: ToggleOpt,
    fobjc_arc_exceptions: ToggleOpt,
    fobjc_avoid_heapify_local_blocks: ToggleOpt,
    fobjc_disable_direct_methods_for_testing: GhostOpt,
    fobjc_encode_cxx_class_template_spec: ToggleOpt,
    fobjc_exceptions: ToggleOpt,
    fobjc_infer_related_result_type: ToggleOpt,
    fobjc_weak: ToggleOpt,
    //     foffload_implicit_host_device_templates: ToggleOpt, // Unknown Argument
    foffload_lto: ToggleOpt,
    foffload_uniform_block: ToggleOpt,
    fomit_frame_pointer: ToggleOpt,
    fopenacc: GhostOpt,
    //     fopenmp: ToggleOpt, // ld.lld: error: unable to find library -lomp
    fopenmp_assume_no_nested_parallelism: GhostOpt,
    fopenmp_assume_no_thread_state: GhostOpt,
    fopenmp_enable_irbuilder: GhostOpt,
    fopenmp_extensions: ToggleOpt,
    fopenmp_force_usm: GhostOpt,
    fopenmp_new_driver: ToggleOpt,
    fopenmp_offload_mandatory: GhostOpt,
    fopenmp_simd: ToggleOpt,
    fopenmp_target_debug: ToggleOpt,
    fopenmp_target_jit: ToggleOpt,
    foperator_names: ToggleOpt,
    foptimize_sibling_calls: ToggleOpt,
    forder_file_instrumentation: GhostOpt,
    fpascal_strings: ToggleOpt,
    //     fpcc_struct_return: GhostOpt, // Unsupported
    fpch_codegen: ToggleOpt,
    fpch_debuginfo: ToggleOpt,
    fpch_instantiate_templates: ToggleOpt,
    fno_pch_validate_input_files_content: GhostOpt,
    fprebuilt_implicit_modules: ToggleOpt,
    fpreserve_as_comments: ToggleOpt,
    fprofile_arcs: ToggleOpt,
    fprofile_generate: ToggleOpt,
    fprofile_instr_generate: ToggleOpt,
    //     fprofile_instr_use: ToggleOpt, // Error in reading profile default.profdata: No such file or directory
    fprofile_sample_accurate: ToggleOpt,
    //     fprotect_parens: ToggleOpt, // Unsupported - weird error msg
    fpseudo_probe_for_profiling: ToggleOpt,
    //     fptrauth_intrinsics: GhostOpt, // Unsupported
    freciprocal_math: ToggleOpt,
    //     freg_struct_return: GhostOpt, // Unsupported
    fregister_global_dtors_with_atexit: ToggleOpt,
    frelaxed_template_template_args: ToggleOpt,
    frtlib_add_rpath: ToggleOpt,
    frtti: ToggleOpt,
    frtti_data: ToggleOpt,
    fsafe_buffer_usage_suggestions: ToggleOpt,
    fsample_profile_use_profi: GhostOpt,
    fsanitize_address_globals_dead_stripping: ToggleOpt,
    fsanitize_address_outline_instrumentation: ToggleOpt,
    fsanitize_address_poison_custom_array_cookie: ToggleOpt,
    fsanitize_address_use_after_scope: ToggleOpt,
    fsanitize_address_use_odr_indicator: ToggleOpt,
    fsanitize_cfi_canonical_jump_tables: ToggleOpt,
    //     fsanitize_cfi_cross_dso: ToggleOpt, // libclang_rt.cfi.a: No such file or directory
    fsanitize_cfi_icall_experimental_normalize_integers: GhostOpt,
    fsanitize_cfi_icall_generalize_pointers: GhostOpt,
    fsanitize_hwaddress_experimental_aliasing: ToggleOpt,
    fno_sanitize_ignorelist: GhostOpt,
    fsanitize_memory_param_retval: ToggleOpt,
    fsanitize_memory_track_origins: ToggleOpt,
    fsanitize_memory_use_after_dtor: ToggleOpt,
    fsanitize_stable_abi: ToggleOpt,
    fsanitize_stats: ToggleOpt,
    fsanitize_thread_atomics: ToggleOpt,
    fsanitize_thread_func_entry_exit: ToggleOpt,
    fsanitize_thread_memory_access: ToggleOpt,
    fsanitize_trap: ToggleOpt,
    fsave_optimization_record: ToggleOpt,
    fseh_exceptions: GhostOpt,
    fshort_enums: ToggleOpt,
    fshort_wchar: ToggleOpt,
    fshow_column: ToggleOpt,
    fshow_skipped_includes: GhostOpt,
    fshow_source_location: ToggleOpt,
    fsigned_char: ToggleOpt,
    fsigned_zeros: ToggleOpt,
    fsized_deallocation: ToggleOpt,
    fsjlj_exceptions: GhostOpt,
    fskip_odr_check_in_gmf: ToggleOpt,
    fslp_vectorize: ToggleOpt,
    fspell_checking: ToggleOpt,
    fsplit_dwarf_inlining: ToggleOpt,
    fsplit_lto_unit: ToggleOpt,
    fno_split_machine_functions: GhostOpt,
    fsplit_stack: ToggleOpt,
    fstack_clash_protection: ToggleOpt,
    fstack_protector: ToggleOpt,
    fstack_protector_all: GhostOpt,
    fstack_protector_strong: GhostOpt,
    fstack_size_section: ToggleOpt,
    fstack_usage: GhostOpt,
    fstandalone_debug: ToggleOpt,
    fstrict_aliasing: ToggleOpt,
    fstrict_enums: ToggleOpt,
    fstrict_float_cast_overflow: ToggleOpt,
    fstrict_return: ToggleOpt,
    fstrict_vtable_pointers: ToggleOpt,
    fsycl: ToggleOpt,
    fsyntax_only: GhostOpt,
    fsystem_module: GhostOpt,
    fno_temp_file: GhostOpt,
    ftest_coverage: ToggleOpt,
    fthreadsafe_statics: ToggleOpt,
    ftime_trace: GhostOpt,
    ftrapv: GhostOpt,
    ftrigraphs: ToggleOpt,
    funified_lto: ToggleOpt,
    funique_basic_block_section_names: ToggleOpt,
    funique_internal_linkage_names: ToggleOpt,
    funique_section_names: ToggleOpt,
    funroll_loops: ToggleOpt,
    funsafe_math_optimizations: ToggleOpt,
    fuse_cxa_atexit: ToggleOpt,
    fuse_init_array: ToggleOpt,
    fuse_line_directives: ToggleOpt,
    fvalidate_ast_input_files_content: GhostOpt,
    fvectorize: ToggleOpt,
    fverbose_asm: ToggleOpt,
    fverify_intermediate_code: ToggleOpt,
    //     fvirtual_function_elimination: ToggleOpt, // Required -flto=full
    //     fvisibility_from_dllstorageclass: ToggleOpt, // Linker fail: undefined reference to `strcmp'
    //     fvisibility_global_new_delete_hidden: GhostOpt, //Deprecated
    fvisibility_inlines_hidden: ToggleOpt,
    fvisibility_inlines_hidden_static_local_var: ToggleOpt,
    fvisibility_ms_compat: GhostOpt,
    fwasm_exceptions: GhostOpt,
    fwhole_program_vtables: ToggleOpt, // Required -flto
    fwrapv: ToggleOpt,
    fwritable_strings: GhostOpt,
    fxl_pragma_pack: ToggleOpt,
    fxray_always_emit_customevents: ToggleOpt,
    fxray_always_emit_typedevents: ToggleOpt,
    fxray_function_index: ToggleOpt,
    fxray_ignore_loops: ToggleOpt,
    fno_xray_instrument: GhostOpt,
    fxray_link_deps: ToggleOpt,
    fzero_initialized_in_bss: ToggleOpt,
    fzvector: ToggleOpt,
    g: GhostOpt,
    gcodeview: GhostOpt,
    gcodeview_command_line: ToggleOpt,
    gcodeview_ghash: ToggleOpt,
    gdwarf: GhostOpt,
    gdwarf_2: GhostOpt,
    gdwarf_3: GhostOpt,
    gdwarf_4: GhostOpt,
    gdwarf_5: GhostOpt,
    gdwarf32: GhostOpt,
    gdwarf64: GhostOpt,
    gembed_source: ToggleOpt,
    //     gen_reproducer: GhostOpt, // Failing because -gen-reproducer is used
    ginline_line_tables: ToggleOpt,
    gline_directives_only: GhostOpt,
    gline_tables_only: GhostOpt,
    gmodules: ToggleOpt,
    gpulibc: GhostOpt,
    gstrict_dwarf: ToggleOpt,
    maix_small_local_exec_tls: GhostOpt,
    //     maltivec: GhostOpt, // Unsupported
    mbackchain: ToggleOpt,
    //     mbranches_within_32B_boundaries: GhostOpt, // Unsupported
    //     mbti_at_return_twice: GhostOpt, // Unknown
    mcabac: GhostOpt,
    mcmse: GhostOpt,
    mconstructor_aliases: ToggleOpt,
    //     mcrbits: GhostOpt, // Unsupported
    pub menable_experimental_extensions: GhostOpt,
    //     mfix_cmse_cve_2021_35465: ToggleOpt, // Unsupported
    //     mfix_cortex_a53_835769: ToggleOpt, // Unsupported
    //     mfix_cortex_a57_aes_1742098: ToggleOpt, // Unsupported
    //     mfix_cortex_a72_aes_1655431: ToggleOpt, // Unsupported
    mno_fmv: GhostOpt,
    mforced_sw_shadow_stack: ToggleOpt,
    mfpxx: GhostOpt,
    mglobal_merge: ToggleOpt,
    //     mhvx: GhostOpt, // Unsupported
    //     mhvx_ieee_fp: GhostOpt, // Unsupported
    //     mhvx_qfloat: GhostOpt, // Unsupported
    mno_iamcu: GhostOpt,
    //     mignore_xcoff_visibility: GhostOpt, // Unsupported
    mimplicit_float: ToggleOpt,
    mincremental_linker_compatible: ToggleOpt,
    mindirect_branch_cs_prefix: GhostOpt,
    //     mips1: GhostOpt,
    //     mips2: GhostOpt,
    //     mips3: GhostOpt,
    //     mips32: GhostOpt,
    //     mips32r2: GhostOpt,
    //     mips32r3: GhostOpt,
    //     mips32r5: GhostOpt,
    //     mips32r6: GhostOpt,
    //     mips4: GhostOpt,
    //     mips5: GhostOpt,
    //     mips64: GhostOpt,
    //     mips64r2: GhostOpt,
    //     mips64r3: GhostOpt,
    //     mips64r5: GhostOpt,
    //     mips64r6: GhostOpt,
    //     mlong_calls: GhostOpt, // Unsupported
    //     mlong_double_128: GhostOpt, // Unsupported
    //     mlong_double_64: GhostOpt, // Unsupported
    //     mlong_double_80: GhostOpt, // Unsupported
    //     mmadd4: GhostOpt, // Unsupported
    mmemops: ToggleOpt,
    mms_bitfields: ToggleOpt,
    //     mno_neg_immediates: GhostOpt, // Unsupported
    //     mnop_mcount: GhostOpt, // Unsupported
    mnvj: ToggleOpt,
    mnvs: ToggleOpt,
    modd_spreg: ToggleOpt,
    //     module_file_info: GhostOpt, // https://github.com/llvm/llvm-project/issues/87852
    momit_leaf_frame_pointer: ToggleOpt,
    //     moutline_atomics: GhostOpt, // Unsupported
    mpackets: ToggleOpt,
    //     mpic_data_is_text_relative: GhostOpt, // Unsupported
    mqdsp6_compat: GhostOpt,
    mrecip: GhostOpt,
    //     mrecord_mcount: GhostOpt, // Unsupported
    //     mregnames: GhostOpt, // Unsupported
    mrelax: ToggleOpt,
    mrelax_all: ToggleOpt,
    //     mrestrict_it: ToggleOpt, // Unknown command line argument '-arm-restrict-it'.
    //     mrtd: ToggleOpt, // error: invalid argument '-fdefault-calling-conv=' not allowed with 'riscv64-unknown-linux-gnu'
    msave_restore: ToggleOpt,
    msoft_float: GhostOpt,
    mstack_arg_probe: ToggleOpt,
    mstackrealign: ToggleOpt,
    mstrict_align: ToggleOpt,
    mtls_direct_seg_refs: ToggleOpt,
    //     mtocdata: GhostOpt, // Unsupported
    //     mvevpu: GhostOpt, // Unsupported

    // Manually added
    E: GhostOpt,
    //     fpic: ToggleOpt,
}

impl LlvmFlags {
    pub fn sanitize(&mut self) {
        //clang: error: invalid argument '-fprofile-generate' not allowed with '-fprofile-instr-generate'
        self.fprofile_generate = if self.fprofile_instr_generate == ToggleOpt::On {
            ToggleOpt::Hidden
        } else {
            self.fprofile_generate
        };
        // //clang: error: invalid argument '-fprofile-instr-use' not allowed with '-fprofile-generate'
        // self.fprofile_instr_use = if self.fprofile_generate == ToggleOpt::On {
        //     ToggleOpt::Hidden
        // } else {
        //     self.fprofile_instr_use
        // };
        //clang: error: invalid argument '-fcs-profile-generate' not allowed with '-fprofile-generate'
        self.fcs_profile_generate = if self.fprofile_generate == ToggleOpt::On {
            GhostOpt::Hidden
        } else {
            self.fcs_profile_generate
        };
        // //clang: error: invalid argument '-fprofile-instr-generate' not allowed with '-fprofile-instr-use'
        // self.fprofile_instr_generate = if self.fprofile_instr_use == ToggleOpt::On {
        //     ToggleOpt::Hidden
        // } else {
        //     self.fprofile_instr_generate
        // };
        //clang: error: invalid argument '-fcoverage-mapping' only allowed with '-fprofile-instr-generate'
        self.fcoverage_mapping = if self.fprofile_instr_generate == ToggleOpt::On {
            self.fcoverage_mapping
        } else {
            ToggleOpt::Hidden
        };
        //clang: error: the combination of '-fno-offload-lto' and '-fopenmp-target-jit' is incompatible
        self.fopenmp_target_jit = if self.foffload_lto == ToggleOpt::Off {
            ToggleOpt::Hidden
        } else {
            self.fopenmp_target_jit
        };
        // //clang: error: invalid argument '-fno-whole-program-vtables' not allowed with '-fvirtual-function-elimination'
        // self.fvirtual_function_elimination = if self.fwhole_program_vtables == ToggleOpt::Off {
        //     ToggleOpt::Hidden
        // } else {
        //     self.fvirtual_function_elimination
        // };
        //clang: error: invalid argument '-gembed-source' only allowed with '-gdwarf-5'
        self.gembed_source = if self.gdwarf_5 == GhostOpt::On {
            self.gembed_source
        } else {
            ToggleOpt::Hidden
        };
        //clang: error: invalid argument '-fno-minimize-whitespace' only allowed with '-E'
        self.fminimize_whitespace = if self.E == GhostOpt::On {
            self.fminimize_whitespace
        } else {
            ToggleOpt::Hidden
        };
        //clang: error: invalid argument '-fkeep-system-includes' only allowed with '-E'
        self.fkeep_system_includes = if self.E == GhostOpt::On {
            self.fkeep_system_includes
        } else {
            ToggleOpt::Hidden
        };
        //clang: error: invalid argument '-fcoverage-mcdc' only allowed with '-fcoverage-mapping'
        self.fcoverage_mcdc = if self.fcoverage_mapping == ToggleOpt::On {
            self.fcoverage_mcdc
        } else {
            ToggleOpt::Hidden
        };
        //clang: error: invalid argument '-gdwarf64' only allowed with 'DWARFv3 or greater'
        self.gdwarf64 = if self.gdwarf_3 == GhostOpt::On
            || self.gdwarf_4 == GhostOpt::On
            || self.gdwarf_5 == GhostOpt::On
        {
            self.gdwarf64
        } else {
            GhostOpt::Hidden
        };

        if self.faddrsig == ToggleOpt::On {
            self.fintegrated_as = ToggleOpt::Hidden;
        }

        if self.gembed_source == ToggleOpt::On {
            self.fintegrated_as = ToggleOpt::Hidden;
        }

        if self.fpseudo_probe_for_profiling == ToggleOpt::On {
            self.fintegrated_as = ToggleOpt::Hidden
        }

        // ??? - Revisit
        if self.ffreestanding == GhostOpt::On {
            self.femit_all_decls = GhostOpt::Hidden;
        }
        if self.fbuiltin == ToggleOpt::Off {
            self.femit_all_decls = GhostOpt::Hidden;
        }
        if self.fms_volatile == ToggleOpt::On {
            self.fms_volatile = ToggleOpt::Hidden;
        }
        if self.fcs_profile_generate == GhostOpt::On
            && self.ffat_lto_objects == ToggleOpt::On
            && self.flto == ToggleOpt::On
        {
            self.fcs_profile_generate = GhostOpt::Hidden;
        }

        // LLD doesn't support split-stack
        if self.flto == ToggleOpt::On {
            self.fsplit_stack = ToggleOpt::Hidden
        }

        if self.flto != ToggleOpt::On {
            self.fwhole_program_vtables = ToggleOpt::Hidden
        }

        // clang: error: -fno-data-sections is not supported with -fembed-bitcode
        // clang: error: -fdebug-types-section is not supported with -fembed-bitcode
        // clang: error: -ffixed-x18 is not supported with -fembed-bitcode
        // clang: error: -fno-unique-basic-block-section-names is not supported with -fembed-bitcode
        // clang: error: -fno-unique-section-names is not supported with -fembed-bitcode
        // clang: error: -mglobal-merge is not supported with -fembed-bitcode
        // clang: error: -mno-relax-all is not supported with -fembed-bitcode
        // clang: error: -mno-stackrealign is not supported with -fembed-bitcode
        // clang: error: -fapple-kext is not supported with -fembed-bitcode
        // clang: error: -ffunction-sections is not supported with -fembed-bitcode
        // clang: error: -fno-unique-internal-linkage-names is not supported with -fembed-bitcode
        if self.fembed_bitcode == GhostOpt::On {
            self.fdata_sections = ToggleOpt::Hidden;
            self.fdebug_types_section = ToggleOpt::Hidden;
            self.ffixed_x18 = GhostOpt::Hidden;
            self.funique_basic_block_section_names = ToggleOpt::Hidden;
            self.funique_section_names = ToggleOpt::Hidden;
            self.mglobal_merge = ToggleOpt::Hidden;
            self.mrelax_all = ToggleOpt::Hidden;
            self.mstackrealign = ToggleOpt::Hidden;
            //     self.fapple_kext = GhostOpt::Hidden;
            self.ffunction_sections = ToggleOpt::Hidden;
            self.funique_internal_linkage_names = ToggleOpt::Hidden;
        }

        // https://github.com/llvm/llvm-project/issues/88038
        if self.fcoverage_mapping == ToggleOpt::On
            && self.fcs_profile_generate == GhostOpt::On
            && self.fprofile_instr_generate == ToggleOpt::On
        {
            self.fcs_profile_generate = GhostOpt::Hidden
        }

        // https://github.com/llvm/llvm-project/issues/88041
        if (self.fembed_bitcode == GhostOpt::On || self.fembed_bitcode_marker == GhostOpt::On)
            && self.ffat_lto_objects == ToggleOpt::On
            && self.flto == ToggleOpt::On
        {
            self.ffat_lto_objects = ToggleOpt::Hidden
        }

        if self.fembed_bitcode == GhostOpt::On
            && self.fsave_optimization_record == ToggleOpt::On
            && self.fintegrated_as == ToggleOpt::Off
        {
            self.fintegrated_as = ToggleOpt::Hidden
        }

        if self.fembed_bitcode == GhostOpt::On
            && (self.gdwarf_2 == GhostOpt::On
                || self.gdwarf_3 == GhostOpt::On
                || self.gdwarf_4 == GhostOpt::On)
            && self.fintegrated_as == ToggleOpt::Off
        {
            self.fintegrated_as = ToggleOpt::Hidden
        }

        // https://github.com/llvm/llvm-project/issues/88046
        if self.fglobal_isel == ToggleOpt::On && self.fstack_protector_all == GhostOpt::On {
            self.fstack_protector_all = GhostOpt::Hidden;
        }

        // https://github.com/llvm/llvm-project/issues/88057
        if self.fglobal_isel == ToggleOpt::On
            && self.finstrument_functions == GhostOpt::On
            && self.flto == ToggleOpt::On
        {
            self.finstrument_functions = GhostOpt::Hidden;
        }

        // https://github.com/llvm/llvm-project/issues/88079
        if self.fstack_protector_all == GhostOpt::On
            || self.fstack_protector_strong == GhostOpt::On
            || self.fstack_protector == ToggleOpt::On
        {
            self.fdirect_access_external_data = ToggleOpt::Hidden;
        }

        // https://github.com/llvm/llvm-project/issues/88153
        if self.gline_directives_only == GhostOpt::On && self.fdebug_macro == ToggleOpt::On {
            self.fdebug_macro = ToggleOpt::Off;
        }

        // https://github.com/llvm/llvm-project/issues/88208
        if self.mms_bitfields == ToggleOpt::On {
            self.mms_bitfields = ToggleOpt::Off;
        }

        // Not expected to work on RISC-V
        self.fglobal_isel = ToggleOpt::Hidden;
    }
}

impl fmt::Display for LlvmFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut flags = self
            .iter()
            .map(|(field_name, field_value)| {
                if field_value.is::<ToggleOpt>() {
                    let value: ToggleOpt = *field_value.downcast_ref().unwrap();
                    let arg_name = field_name.replace('_', "-");
                    let arg_name = if arg_name == "fchar8-t" {
                        "fchar8_t".to_string()
                    } else {
                        arg_name
                    };
                    match value {
                        ToggleOpt::Hidden => "".to_string(),
                        ToggleOpt::Off => format!("-{}no-{}", &arg_name[0..1], &arg_name[1..]),
                        ToggleOpt::On => format!("-{}", arg_name),
                    }
                } else if field_value.is::<GhostOpt>() {
                    let value: GhostOpt = *field_value.downcast_ref().unwrap();
                    let arg_name = field_name.replace('_', "-");
                    let arg_name = if arg_name == "fno-modules-validate-input-files-content" {
                        "fno_modules-validate-input-files-content".to_string()
                    } else if arg_name == "fno-pch-validate-input-files-content" {
                        "fno_pch-validate-input-files-content".to_string()
                    } else {
                        arg_name
                    };
                    match value {
                        GhostOpt::Hidden => "".to_string(),
                        GhostOpt::On => format!("-{}", arg_name),
                    }
                } else {
                    panic!("Unknown datatype for field: {}", field_name)
                }
            })
            .collect::<Vec<_>>();

        if self.flto == ToggleOpt::On {
            // Use lld
            let mut lld = vec!["-fuse-ld=lld".to_string()];
            flags.append(&mut lld)
        }

        write!(f, "{}", flags.join(" "))
    }
}
