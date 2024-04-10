use arbitrary::Arbitrary;
use std::fmt;
use struct_iterable::Iterable;

use crate::{GhostOpt, ToggleOpt};

#[derive(Arbitrary, Debug, Iterable, Clone)]
pub struct GccFlags {
    pub toggles: GccToggles,
    pub riscv_toggles: GccRiscvToggles,
}

impl GccFlags {
    pub fn sanitize(&mut self) {
        self.toggles.sanitize();
        self.riscv_toggles.sanitize();
    }
}

impl fmt::Display for GccFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.toggles, self.riscv_toggles)
    }
}

#[allow(non_camel_case_types)]
#[derive(Arbitrary, Debug, Iterable, Clone)]
pub struct GccToggles {
    // Optimization Flags
    faggressive_loop_optimizations: ToggleOpt,
    fallocation_dce: ToggleOpt,
    fallow_store_data_races: ToggleOpt,
    fassociative_math: ToggleOpt,
    fauto_inc_dec: ToggleOpt,
    // fauto_profile: ToggleOpt,
    fbranch_count_reg: ToggleOpt,
    fbranch_probabilities: ToggleOpt,
    fcaller_saves: ToggleOpt,
    fcode_hoisting: ToggleOpt,
    fcombine_stack_adjustments: ToggleOpt,
    fcompare_elim: ToggleOpt,
    fconserve_stack: ToggleOpt,
    fcprop_registers: ToggleOpt,
    fcrossjumping: ToggleOpt,
    fcse_follow_jumps: ToggleOpt,
    fcse_skip_blocks: ToggleOpt,
    fcx_fortran_rules: ToggleOpt,
    fcx_limited_range: ToggleOpt,
    fdata_sections: ToggleOpt,
    fdce: ToggleOpt,
    fdefer_pop: ToggleOpt,
    fdelayed_branch: ToggleOpt,
    fdelete_null_pointer_checks: ToggleOpt,
    fdevirtualize: ToggleOpt,
    fdevirtualize_at_ltrans: ToggleOpt,
    fdevirtualize_speculatively: ToggleOpt,
    fdse: ToggleOpt,
    fearly_inlining: ToggleOpt,
    fexpensive_optimizations: ToggleOpt,
    ffast_math: ToggleOpt,
    ffat_lto_objects: ToggleOpt,
    ffinite_loops: ToggleOpt,
    ffinite_math_only: ToggleOpt,
    ffloat_store: ToggleOpt,
    ffold_mem_offsets: ToggleOpt,
    fforward_propagate: ToggleOpt,
    ffp_int_builtin_inexact: ToggleOpt,
    ffunction_cse: ToggleOpt,
    ffunction_sections: ToggleOpt,
    fgcse: ToggleOpt,
    fgcse_after_reload: ToggleOpt,
    fgcse_las: ToggleOpt,
    fgcse_lm: ToggleOpt,
    fgcse_sm: ToggleOpt,
    fgraphite_identity: ToggleOpt,
    fguess_branch_probability: ToggleOpt,
    fhoist_adjacent_loads: ToggleOpt,
    fif_conversion: ToggleOpt,
    fif_conversion2: ToggleOpt,
    findirect_inlining: ToggleOpt,
    finline: ToggleOpt,
    finline_functions: ToggleOpt,
    finline_functions_called_once: ToggleOpt,
    finline_small_functions: ToggleOpt,
    fipa_bit_cp: ToggleOpt,
    fipa_cp: ToggleOpt,
    fipa_cp_clone: ToggleOpt,
    fipa_icf: ToggleOpt,
    fipa_modref: ToggleOpt,
    fipa_profile: ToggleOpt,
    fipa_pta: ToggleOpt,
    fipa_pure_const: ToggleOpt,
    fipa_ra: ToggleOpt,
    fipa_reference: ToggleOpt,
    fipa_reference_addressable: ToggleOpt,
    fipa_sra: ToggleOpt,
    fipa_stack_alignment: ToggleOpt,
    fipa_strict_aliasing: ToggleOpt,
    fipa_vrp: ToggleOpt,
    fira_hoist_pressure: ToggleOpt,
    fira_loop_pressure: ToggleOpt,
    fira_share_save_slots: ToggleOpt,
    fira_share_spill_slots: ToggleOpt,
    fisolate_erroneous_paths_attribute: ToggleOpt,
    fisolate_erroneous_paths_dereference: ToggleOpt,
    fivopts: ToggleOpt,
    fkeep_inline_functions: ToggleOpt,
    fkeep_static_consts: ToggleOpt,
    fkeep_static_functions: ToggleOpt,
    flimit_function_alignment: ToggleOpt,
    flive_range_shrinkage: ToggleOpt,
    floop_block: ToggleOpt,
    floop_interchange: ToggleOpt,
    floop_nest_optimize: ToggleOpt,
    floop_parallelize_all: ToggleOpt,
    floop_strip_mine: ToggleOpt,
    floop_unroll_and_jam: ToggleOpt,
    flra_remat: ToggleOpt,
    flto: ToggleOpt,
    fmath_errno: ToggleOpt,
    fmerge_all_constants: ToggleOpt,
    fmerge_constants: ToggleOpt,
    fmodulo_sched: ToggleOpt,
    fmodulo_sched_allow_regmoves: ToggleOpt,
    fmove_loop_invariants: ToggleOpt,
    fmove_loop_stores: ToggleOpt,
    fomit_frame_pointer: ToggleOpt,
    foptimize_sibling_calls: ToggleOpt,
    fpartial_inlining: ToggleOpt,
    fpeel_loops: ToggleOpt,
    fpeephole: ToggleOpt,
    fpeephole2: ToggleOpt,
    fpredictive_commoning: ToggleOpt,
    fprefetch_loop_arrays: ToggleOpt,
    fprintf_return_value: ToggleOpt,
    fprofile_correction: ToggleOpt,
    fprofile_partial_training: ToggleOpt,
    fprofile_reorder_functions: ToggleOpt,
    fprofile_use: ToggleOpt,
    fprofile_values: ToggleOpt,
    freciprocal_math: ToggleOpt,
    free: ToggleOpt,
    frename_registers: ToggleOpt,
    freorder_blocks: ToggleOpt,
    freorder_blocks_and_partition: ToggleOpt,
    freorder_functions: ToggleOpt,
    frerun_cse_after_loop: ToggleOpt,
    freschedule_modulo_scheduled_loops: ToggleOpt,
    frounding_math: ToggleOpt,
    fsave_optimization_record: ToggleOpt,
    fsched_critical_path_heuristic: ToggleOpt,
    fsched_dep_count_heuristic: ToggleOpt,
    fsched_group_heuristic: ToggleOpt,
    fsched_interblock: ToggleOpt,
    fsched_last_insn_heuristic: ToggleOpt,
    fsched_pressure: ToggleOpt,
    fsched_rank_heuristic: ToggleOpt,
    fsched_spec: ToggleOpt,
    fsched_spec_insn_heuristic: ToggleOpt,
    fsched_spec_load: ToggleOpt,
    fsched_spec_load_dangerous: ToggleOpt,
    fsched2_use_superblocks: ToggleOpt,
    fschedule_fusion: ToggleOpt,
    fschedule_insns: ToggleOpt,
    fschedule_insns2: ToggleOpt,
    fsection_anchors: ToggleOpt,
    fsel_sched_pipelining: ToggleOpt,
    fsel_sched_pipelining_outer_loops: ToggleOpt,
    fselective_scheduling: ToggleOpt,
    fselective_scheduling2: ToggleOpt,
    fsemantic_interposition: ToggleOpt,
    fshrink_wrap: ToggleOpt,
    fshrink_wrap_separate: ToggleOpt,
    fsignaling_nans: ToggleOpt,
    fsigned_zeros: ToggleOpt,
    fsingle_precision_constant: ToggleOpt,
    fsplit_ivs_in_unroller: ToggleOpt,
    fsplit_loops: ToggleOpt,
    fsplit_paths: ToggleOpt,
    fsplit_wide_types: ToggleOpt,
    fsplit_wide_types_early: ToggleOpt,
    fssa_backprop: ToggleOpt,
    fssa_phiopt: ToggleOpt,
    fstdarg_opt: ToggleOpt,
    fstore_merging: ToggleOpt,
    fstrict_aliasing: ToggleOpt,
    fthread_jumps: ToggleOpt,
    ftoplevel_reorder: ToggleOpt,
    ftracer: ToggleOpt,
    ftrapping_math: ToggleOpt,
    ftree_bit_ccp: ToggleOpt,
    ftree_builtin_call_dce: ToggleOpt,
    ftree_ccp: ToggleOpt,
    ftree_ch: ToggleOpt,
    ftree_coalesce_vars: ToggleOpt,
    ftree_copy_prop: ToggleOpt,
    ftree_dce: ToggleOpt,
    ftree_dominator_opts: ToggleOpt,
    ftree_dse: ToggleOpt,
    ftree_forwprop: ToggleOpt,
    ftree_fre: ToggleOpt,
    ftree_loop_distribute_patterns: ToggleOpt,
    ftree_loop_distribution: ToggleOpt,
    ftree_loop_if_convert: ToggleOpt,
    ftree_loop_im: ToggleOpt,
    ftree_loop_ivcanon: ToggleOpt,
    ftree_loop_linear: ToggleOpt,
    ftree_loop_optimize: ToggleOpt,
    ftree_loop_vectorize: ToggleOpt,
    ftree_partial_pre: ToggleOpt,
    ftree_phiprop: ToggleOpt,
    ftree_pre: ToggleOpt,
    ftree_pta: ToggleOpt,
    ftree_reassoc: ToggleOpt,
    ftree_scev_cprop: ToggleOpt,
    ftree_sink: ToggleOpt,
    ftree_slsr: ToggleOpt,
    ftree_sra: ToggleOpt,
    ftree_switch_conversion: ToggleOpt,
    ftree_tail_merge: ToggleOpt,
    ftree_ter: ToggleOpt,
    ftree_vectorize: ToggleOpt,
    ftree_vrp: ToggleOpt,
    funconstrained_commons: ToggleOpt,
    funit_at_a_time: ToggleOpt,
    funroll_all_loops: ToggleOpt,
    funroll_loops: ToggleOpt,
    funsafe_math_optimizations: ToggleOpt,
    funswitch_loops: ToggleOpt,
    // fuse_linker_plugin: ToggleOpt,
    fvariable_expansion_in_unroller: ToggleOpt,
    fvect_cost_model: ToggleOpt,
    fvpt: ToggleOpt,
    fweb: ToggleOpt,
    fwhole_program: ToggleOpt,
    fwpa: ToggleOpt,
    fzero_initialized_in_bss: ToggleOpt,

    // Debug flags
    fdebug_types_section: ToggleOpt,
    fdwarf2_cfi_asm: ToggleOpt,
    feliminate_unused_debug_symbols: ToggleOpt,
    feliminate_unused_debug_types: ToggleOpt,
    femit_class_debug_always: ToggleOpt,
    femit_struct_debug_baseonly: ToggleOpt,
    femit_struct_debug_reduced: ToggleOpt,
    fmerge_debug_strings: ToggleOpt,
    fvar_tracking: ToggleOpt,
    fvar_tracking_assignments: ToggleOpt,
    g: GhostOpt,
    gas_loc_support: ToggleOpt,
    gas_locview_support: ToggleOpt,
    gbtf: GhostOpt,
    // gcodeview: GhostOpt, // Linker doesn't like this one - windows debug format
    gcolumn_info: ToggleOpt,
    gctf: GhostOpt,
    gdescribe_dies: ToggleOpt,
    gdwarf: GhostOpt,
    gdwarf32: GhostOpt,
    gdwarf64: GhostOpt,
    ggdb: GhostOpt,
    ginline_points: ToggleOpt,
    ginternal_reset_location_views: ToggleOpt,
    grecord_gcc_switches: ToggleOpt,
    gsplit_dwarf: ToggleOpt,
    gstatement_frontiers: ToggleOpt,
    gstrict_dwarf: ToggleOpt,
    gvariable_location_views: ToggleOpt,
    // gvms: GhostOpt, // not supported on riscv

    // Developer flags
    fchecking: ToggleOpt,
    fcompare_debug_second: GhostOpt,
    fdbg_cnt_list: ToggleOpt,
    fdump_debug: GhostOpt,
    fdump_earlydebug: GhostOpt,
    fdump_ipa_all: GhostOpt,
    fdump_ipa_cgraph: GhostOpt,
    fdump_ipa_inline: GhostOpt,
    fdump_lang_all: GhostOpt,
    fdump_noaddr: ToggleOpt,
    fdump_passes: ToggleOpt,
    fdump_statistics: GhostOpt,
    fdump_tree_all: GhostOpt,
    fdump_unnumbered: ToggleOpt,
    fdump_unnumbered_links: ToggleOpt,
    flto_report: ToggleOpt,
    flto_report_wpa: ToggleOpt,
    fmem_report: ToggleOpt,
    fmem_report_wpa: ToggleOpt,
    fmultiflags: GhostOpt,
    fopt_info: ToggleOpt,
    fpost_ipa_mem_report: ToggleOpt,
    fpre_ipa_mem_report: ToggleOpt,
    fprofile_report: ToggleOpt,
    fstack_usage: GhostOpt,
    fstats: ToggleOpt,
    ftime_report: ToggleOpt,
    ftime_report_details: ToggleOpt,
    fvar_tracking_assignments_toggle: ToggleOpt,
    gtoggle: ToggleOpt,
}

impl GccToggles {
    pub fn sanitize(&mut self) {
        self.ftoplevel_reorder = if self.funit_at_a_time == ToggleOpt::On {
            self.ftoplevel_reorder
        } else {
            ToggleOpt::Hidden
        };
        self.fsection_anchors = if self.funit_at_a_time == ToggleOpt::On {
            self.fsection_anchors
        } else {
            ToggleOpt::Hidden
        };
        self.fsection_anchors = if self.ftoplevel_reorder == ToggleOpt::Off {
            ToggleOpt::Off
        } else {
            self.fsection_anchors
        };
        self.fwpa = if self.fwpa == ToggleOpt::On {
            ToggleOpt::Hidden
        } else {
            self.fwpa
        };
        self.gbtf = if self.gctf == GhostOpt::On {
            GhostOpt::Hidden
        } else {
            self.gbtf
        };
    }
}

impl fmt::Display for GccToggles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let flags = self
            .iter()
            .map(|(field_name, field_value)| {
                if field_value.is::<ToggleOpt>() {
                    let value: ToggleOpt = *field_value.downcast_ref().unwrap();
                    let arg_name = field_name.replace('_', "-");
                    match value {
                        ToggleOpt::Hidden => "".to_string(),
                        ToggleOpt::Off => format!("-{}no-{}", &arg_name[0..1], &arg_name[1..]),
                        ToggleOpt::On => format!("-{}", arg_name),
                    }
                } else if field_value.is::<GhostOpt>() {
                    let value: GhostOpt = *field_value.downcast_ref().unwrap();
                    let arg_name = field_name.replace('_', "-");
                    match value {
                        GhostOpt::Hidden => "".to_string(),
                        GhostOpt::On => format!("-{}", arg_name),
                    }
                } else {
                    panic!("Unknown datatype for field: {}", field_name)
                }
            })
            .collect::<Vec<_>>();

        write!(f, "{}", flags.join(" "))
    }
}

#[allow(non_camel_case_types)]
#[derive(Arbitrary, Debug, Iterable, Clone)]
pub struct GccRiscvToggles {
    pub mbig_endian: GhostOpt,
    mlittle_endian: GhostOpt,
    mcsr_check: ToggleOpt,
    pub mdiv: ToggleOpt,
    mexplicit_relocs: ToggleOpt,
    mfdiv: ToggleOpt,
    minline_atomics: ToggleOpt,
    minline_strcmp: ToggleOpt,
    minline_strlen: ToggleOpt,
    minline_strncmp: ToggleOpt,
    mmovcc: ToggleOpt,
    mplt: ToggleOpt,
    mrelax: ToggleOpt,
    mriscv_attribute: ToggleOpt,
    msave_restore: ToggleOpt,
    mshorten_memrefs: ToggleOpt,
    mstrict_align: ToggleOpt,
}

impl GccRiscvToggles {
    pub fn sanitize(&mut self) {}
}

impl fmt::Display for GccRiscvToggles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let flags = self
            .iter()
            .map(|(field_name, field_value)| {
                if field_value.is::<ToggleOpt>() {
                    let value: ToggleOpt = *field_value.downcast_ref().unwrap();
                    let arg_name = field_name.replace('_', "-");
                    match value {
                        ToggleOpt::Hidden => "".to_string(),
                        ToggleOpt::Off => format!("-{}no-{}", &arg_name[0..1], &arg_name[1..]),
                        ToggleOpt::On => format!("-{}", arg_name),
                    }
                } else if field_value.is::<GhostOpt>() {
                    let value: GhostOpt = *field_value.downcast_ref().unwrap();
                    let arg_name = field_name.replace('_', "-");
                    match value {
                        GhostOpt::Hidden => "".to_string(),
                        GhostOpt::On => format!("-{}", arg_name),
                    }
                } else {
                    panic!("Unknown datatype for field: {}", field_name)
                }
            })
            .collect::<Vec<_>>();

        write!(f, "{}", flags.join(" "))
    }
}
