mod slint_generatedMainWindow {
     # ! [allow (non_snake_case , non_camel_case_types)] # ! [allow (unused_braces , unused_parens)] # ! [allow (clippy :: all , clippy :: pedantic , clippy :: nursery)] # ! [allow (unknown_lints , if_let_rescope , tail_expr_drop_order)] use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     # [allow (dead_code)] # [derive (Default , Copy , Clone , PartialEq , Debug)] pub enum r#GameState {
         # [default] r#Initial , r#Normal , r#Lose , r#Win , }
     # [derive (Default , PartialEq , Debug , Clone)] pub struct r#Position {
         pub r#col : i32 , pub r#row : i32 }
     # [derive (Default , PartialEq , Debug , Clone)] pub struct r#Tile {
         pub r#flagged : bool , pub r#position : r#Position , pub r#value : i32 , pub r#visible : bool }
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_14_1 = slint :: VersionCheck_1_14_1 ;
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerFluentPalette_70 {
         r#accent_control_border : sp :: Property < slint :: Brush > , r#background : sp :: Property < slint :: Brush > , r#color_scheme : sp :: Property < sp :: r#ColorScheme > , r#dark_color_scheme : sp :: Property < bool > , r#foreground : sp :: Property < slint :: Brush > , globals : sp :: OnceCell < sp :: Weak < SharedGlobals >> , }
     impl InnerFluentPalette_70 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , globals : & sp :: Rc < SharedGlobals >) {
             # ! [allow (unused)] let _ = self . globals . set (sp :: Rc :: downgrade (globals)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_70 :: FIELD_OFFSETS . r#accent_control_border }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                         (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9067000000000001f64 as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((603979776f64) as u32) , position : 1f64 as _ }
                        ]))) as _ }
                     else {
                         slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((352321535f64) as u32) , position : 0.9067000000000001f64 as _ }
                         , sp :: GradientStop {
                             color : sp :: Color :: from_argb_encoded ((1711276032f64) as u32) , position : 1f64 as _ }
                        ])) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_70 :: FIELD_OFFSETS . r#background }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4280032284f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294638330f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_70 :: FIELD_OFFSETS . r#color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . color_scheme ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     ({
                         let r#tmp_FluentPalette_70_color_scheme = InnerFluentPalette_70 :: FIELD_OFFSETS . r#color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () ;
                         if ! (((r#tmp_FluentPalette_70_color_scheme . clone ())) == ((sp :: r#ColorScheme :: r#Unknown))) {
                             ((((r#tmp_FluentPalette_70_color_scheme . clone ())) == ((sp :: r#ColorScheme :: r#Dark)))) as _ }
                         else {
                             ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () . window ()) . color_scheme ())) == ((sp :: r#ColorScheme :: r#Dark)) }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerFluentPalette_70 :: FIELD_OFFSETS . r#foreground }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . upgrade () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerAboutSlint_root_1 {
         r#root_1 : sp :: r#Empty , r#image_3 : sp :: r#ImageItem , r#text_4 : sp :: r#SimpleText , r#root_1_height : sp :: Property < sp :: LogicalLength > , r#root_1_i_layout_2_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_i_layout_2_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_i_layout_2_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_width : sp :: Property < sp :: LogicalLength > , r#root_1_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerAboutSlint_root_1 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerAboutSlint_root_1 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Start as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = sp :: Item :: layout_info (({
                                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#image_3 }
                                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1)) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 48f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_height }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 8f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#image_3 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1)) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = 256f64 as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = sp :: Item :: layout_info (({
                                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#image_3 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1)) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 48f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 2u32 - 1)) as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 12f64 as _ ;
                         the_struct . r#end = 12f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Start as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_0 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_0 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_0 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (({
                                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layoutinfo_h }
                            ) . apply_pin (_self) . get ()) . r#min as _ ;
                             the_struct . r#min_percent = (r#layout_info_0 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_0 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_0 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    )) + ((({
                         * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_1 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_1 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_1 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (({
                                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layoutinfo_v }
                            ) . apply_pin (_self) . get ()) . r#min as _ ;
                             the_struct . r#min_percent = (r#layout_info_1 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_1 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = (r#layout_info_1 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    )) + ((({
                         * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#image_3 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#image_3 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#image_3 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((InnerFluentPalette_70 :: FIELD_OFFSETS . r#color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get ())) == ((sp :: r#ColorScheme :: r#Dark)) {
                         (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                     else {
                         sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg")) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#image_3 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((({
                         * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_width }
                    ) . apply_pin (_self) . get () . get ()) as f64) - ((12f64) as f64)) as f64) - ((12f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerFluentPalette_70 :: FIELD_OFFSETS . r#foreground . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Version 1.14.1\nhttps://slint.dev/")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((({
                         * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_width }
                    ) . apply_pin (_self) . get () . get ()) as f64) - ((12f64) as f64)) as f64) - ((12f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#image_3 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#image_3 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#image_3 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (({
                             * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (({
                             * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , ({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , (((((({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((12f64) as f64)) as f64) - ((12f64) as f64)) as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , (((((({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((12f64) as f64)) as f64) - ((12f64) as f64)) as sp :: Coord , 12f64 as sp :: Coord , ({
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Image , 2u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("#MadeWithSlint")) , (2u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (sp :: SharedString :: from ("Version 1.14.1\nhttps://slint.dev/")) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerFocusBorder_root_5 {
         r#root_5 : sp :: r#BasicBorderRectangle , r#rectangle_6 : sp :: r#BasicBorderRectangle , r#root_5_height : sp :: Property < sp :: LogicalLength > , r#root_5_width : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerFocusBorder_root_5 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerFocusBorder_root_5 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                    )) as _ }
                ) ;
                 }
             ({
                 * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#rectangle_6 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((3003121664f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#rectangle_6 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
                     + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . get () . get ()) as f64) - ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#rectangle_6 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#rectangle_6 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#rectangle_6 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => ((((({
                     * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((4f64) as f64)) as sp :: Coord , (((({
                     * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((4f64) as f64)) as sp :: Coord , 2f64 as sp :: Coord , 2f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerButton_root_7 {
         r#root_7 : sp :: r#Empty , r#i_background_8 : sp :: r#BasicBorderRectangle , r#i_border_9 : sp :: r#BasicBorderRectangle , r#i_touch_area_15 : sp :: r#TouchArea , r#i_focus_scope_16 : sp :: r#FocusScope , r#root_7_checked : sp :: Property < bool > , r#root_7_has_focus : sp :: Property < bool > , r#root_7_height : sp :: Property < sp :: LogicalLength > , r#root_7_i_background_8_width : sp :: Property < sp :: LogicalLength > , r#root_7_i_layout_10_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_7_i_layout_10_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_7_i_layout_10_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_7_icon : sp :: Property < sp :: Image > , r#root_7_icon_size : sp :: Property < sp :: LogicalLength > , r#root_7_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_7_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_7_pressed : sp :: Property < bool > , r#root_7_primary : sp :: Property < bool > , r#root_7_state : sp :: Property < i32 > , r#root_7_text : sp :: Property < sp :: SharedString > , r#root_7_text_color : sp :: Property < slint :: Brush > , r#root_7_width : sp :: Property < sp :: LogicalLength > , r#root_7_x : sp :: Property < sp :: LogicalLength > , r#root_7_y : sp :: Property < sp :: LogicalLength > , r#root_7_accessible_action_default : sp :: Callback < () , () > , r#root_7_clicked : sp :: Callback < () , () > , repeater0 : sp :: Conditional < InnerComponent_image_11 > , repeater1 : sp :: Conditional < InnerComponent_text_13 > , repeater2 : sp :: Conditional < InnerComponent_focusborder_17 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerButton_root_7 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerButton_root_7 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_7_icon = ({
                             * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon }
                        ) . apply_pin (_self) . get () ;
                         (((((r#tmp_root_7_icon . clone () . size ()) . r#width) as f64) > ((0f64) as f64))) && (((((r#tmp_root_7_icon . clone () . size ()) . r#height) as f64) > ((0f64) as f64))) }
                    ) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text }
                    ) . apply_pin (_self) . get ())) != ((sp :: SharedString :: from (""))))) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_has_focus }
                    ) . apply_pin (_self) . get ())) && ((({
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ())))) as _ }
                 }
            ) ;
             sp :: Property :: link_two_way (({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_touch_area_15 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self)) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_accessible_action_default }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_touch_area_15 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_has_focus }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_background_8_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_7 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_11 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_7 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         r#repeated_indices [1usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [1usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 12f64 as _ ;
                                 the_struct . r#end = 12f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_background_8_width }
                            ) . apply_pin (_self) . get () . get () as _ , r#spacing : 4f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_7 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_11 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerButton_root_7 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 4f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerButton_root_7 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_11 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         InnerButton_root_7 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 5f64 as _ ;
                             the_struct . r#end = 5f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon_size }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_2 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_2 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_2 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_h }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                             the_struct . r#min_percent = (r#layout_info_2 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_2 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = 0f64 as _ ;
                             the_struct }
                         }
                    )) + (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + ((({
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_3 = {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                         ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_3 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_3 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_v }
                            ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                             the_struct . r#min_percent = (r#layout_info_3 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = (r#layout_info_3 . clone ()) . r#preferred as _ ;
                             the_struct . r#stretch = 0f64 as _ ;
                             the_struct }
                         }
                    )) + (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + ((({
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_pressed }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ())) && ((({
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_touch_area_15 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ! ({
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_pressed }
                        ) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             if ({
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_touch_area_15 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                                 (3f64) as _ }
                             else {
                                 if ({
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                                ) . apply_pin (_self) . get () {
                                     (4f64) as _ }
                                 else {
                                     0f64 }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_7_state = ({
                             * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_state }
                        ) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_7_state . clone () as f64) , & (1f64 as f64)) {
                             (if ((({
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_primary }
                            ) . apply_pin (_self) . get ())) || ((({
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                            ) . apply_pin (_self) . get ())) {
                                 (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((2281701375f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((1593835519f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1577058304f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_7_state . clone () as f64) , & (2f64 as f64)) {
                                 (if ((({
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_primary }
                                ) . apply_pin (_self) . get ())) || ((({
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                                ) . apply_pin (_self) . get ())) {
                                     (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((2147483648f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                    )) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_7_state . clone () as f64) , & (4f64 as f64)) {
                                     (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                    )) as _ }
                                 else {
                                     if ((({
                                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_primary }
                                    ) . apply_pin (_self) . get ())) || ((({
                                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                                    ) . apply_pin (_self) . get ())) {
                                         (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4278190080f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((4294967295f64) as u32) }
                                        )) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4294967295f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((3858759680f64) as u32) }
                                        ) }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_background_8 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_7_state = ({
                             * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_state }
                        ) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_7_state . clone () as f64) , & (1f64 as f64)) {
                             (if ((({
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_primary }
                            ) . apply_pin (_self) . get ())) || ((({
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                            ) . apply_pin (_self) . get ())) {
                                 (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((704643071f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((939524096f64) as u32) }
                                )) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((184549375f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_7_state . clone () as f64) , & (2f64 as f64)) {
                                 (if ((({
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_primary }
                                ) . apply_pin (_self) . get ())) || ((({
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                                ) . apply_pin (_self) . get ())) {
                                     (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((3428896255f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((3422576568f64) as u32) }
                                    )) as _ }
                                 else {
                                     slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                         (sp :: Color :: from_argb_encoded ((150994943f64) as u32)) as _ }
                                     else {
                                         sp :: Color :: from_argb_encoded ((1308228089f64) as u32) }
                                    ) }
                                ) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_7_state . clone () as f64) , & (3f64 as f64)) {
                                     (if ((({
                                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_primary }
                                    ) . apply_pin (_self) . get ())) || ((({
                                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                                    ) . apply_pin (_self) . get ())) {
                                         (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((3865103871f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((3858784184f64) as u32) }
                                        )) as _ }
                                     else {
                                         slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((2163866105f64) as u32) }
                                        ) }
                                    ) as _ }
                                 else {
                                     if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_7_state . clone () as f64) , & (4f64 as f64)) {
                                         (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                             (sp :: Color :: from_argb_encoded ((4284534271f64) as u32)) as _ }
                                         else {
                                             sp :: Color :: from_argb_encoded ((4278214584f64) as u32) }
                                        )) as _ }
                                     else {
                                         if ({
                                             * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_primary }
                                        ) . apply_pin (_self) . get () {
                                             (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                                 (sp :: Color :: from_argb_encoded ((4284534271f64) as u32)) as _ }
                                             else {
                                                 sp :: Color :: from_argb_encoded ((4278214584f64) as u32) }
                                            )) as _ }
                                         else {
                                             slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                                 (sp :: Color :: from_argb_encoded ((268435455f64) as u32)) as _ }
                                             else {
                                                 sp :: Color :: from_argb_encoded ((3019898879f64) as u32) }
                                            ) }
                                         }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_background_8 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_border_9 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_7_state = ({
                             * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_state }
                        ) . apply_pin (_self) . get () ;
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_7_state . clone () as f64) , & (1f64 as f64)) {
                             (if ((({
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_primary }
                            ) . apply_pin (_self) . get ())) || ((({
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                            ) . apply_pin (_self) . get ())) {
                                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as _ }
                             else {
                                 slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                                ) }
                            ) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_7_state . clone () as f64) , & (2f64 as f64)) {
                                 (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                     (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                                 else {
                                     sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                                )) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& (r#tmp_root_7_state . clone () as f64) , & (4f64 as f64)) {
                                     (InnerFluentPalette_70 :: FIELD_OFFSETS . r#accent_control_border . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get ()) as _ }
                                 else {
                                     if ({
                                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_primary }
                                    ) . apply_pin (_self) . get () {
                                         (InnerFluentPalette_70 :: FIELD_OFFSETS . r#accent_control_border . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get ()) as _ }
                                     else {
                                         if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                                             (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                                 color : sp :: Color :: from_argb_encoded ((402653183f64) as u32) , position : 0f64 as _ }
                                             , sp :: GradientStop {
                                                 color : sp :: Color :: from_argb_encoded ((301989888f64) as u32) , position : 0.0833f64 as _ }
                                            ]))) as _ }
                                         else {
                                             slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (180f64 as _ , [sp :: GradientStop {
                                                 color : sp :: Color :: from_argb_encoded ((251658240f64) as u32) , position : 0.9058f64 as _ }
                                             , sp :: GradientStop {
                                                 color : sp :: Color :: from_argb_encoded ((687865856f64) as u32) , position : 1f64 as _ }
                                            ])) }
                                         }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_border_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_border_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_touch_area_15 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if false {
                                 ({
                                     ({
                                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                                    ) . apply_pin (_self) . set ((! ({
                                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                                    ) . apply_pin (_self) . get ()) as _) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             ;
                             ({
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_clicked }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ! ((((((args . 0 . clone ()) . r#text)) == ((sp :: SharedString :: from (" "))))) || (((((args . 0 . clone ()) . r#text)) == ((sp :: SharedString :: from ("\n")))))) {
                                 ({
                                     {
                                         sp :: r#EventResult :: r#Reject }
                                     }
                                ) as _ }
                             else {
                                 {
                                     ({
                                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_touch_area_15 }
                                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& ()) ;
                                     sp :: r#EventResult :: r#Accept }
                                 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_background_8 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_background_8 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_background_8 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_border_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_border_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_border_9 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_touch_area_15 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_click) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#focus_on_tab_navigation) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_7 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_11 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerButton_root_7 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerButton_root_7 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_17 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                             * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_h }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                             * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_v }
                        ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_7 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_11 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerButton_root_7 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerButton_root_7 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_17 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerButton_root_7 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_11 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerButton_root_7 :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_13 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerButton_root_7 :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_focusborder_17 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 3u32 => (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 5u32 => (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Button , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text }
                ) . apply_pin (_self) . get ()) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: SupportedAccessibilityAction :: r#Default , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_clear_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1) , false , sp :: FocusReason :: Programmatic)) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1) , true , sp :: FocusReason :: Programmatic)) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_11 {
         r#image_11 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_11 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_7 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_11 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _ = (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon) . apply_pin (x . as_pin_ref ()))) . map (| x | sp :: Property :: link_two_way (({
                     * & InnerComponent_image_11 :: FIELD_OFFSETS . r#image_11 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , x)) ;
                 }
             ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_11 :: FIELD_OFFSETS . r#image_11 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_11 :: FIELD_OFFSETS . r#image_11 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_11 :: FIELD_OFFSETS . r#image_11 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_11 :: FIELD_OFFSETS . r#image_11 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_11 :: FIELD_OFFSETS . r#image_11 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_11 :: FIELD_OFFSETS . r#image_11 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerComponent_image_11 :: FIELD_OFFSETS . r#image_11 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerComponent_image_11 :: FIELD_OFFSETS . r#image_11 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon_size) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , 5f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_image_11 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_7 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_7 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_11 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_image_11 :: FIELD_OFFSETS . r#image_11 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_11) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_11 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_image_11 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_11 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_11 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 6u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_image_11 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_11 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_13 {
         r#text_13 : sp :: r#SimpleText , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_13 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_7 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_13 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text_color) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new ((((1.0766f64) as f64) * ((28f64) as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (((400f64) as i32)) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (((((((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get ()) as f64) - ((5f64) as f64)) as f64) - ((5f64) as f64)) as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                     let cache = x . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                ) . unwrap_or_default () as sp :: Coord , 5f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_text_13 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_7 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_7 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_13 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_text_13 :: FIELD_OFFSETS . r#text_13 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_13) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_13 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_text_13 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_13 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_13 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 7u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_text_13 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_13 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_focusborder_17 {
         r#focusborder_17 : InnerFocusBorder_root_5 , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_focusborder_17 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_7 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_focusborder_17 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerFocusBorder_root_5 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#focusborder_17 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index , tree_index_of_first_child + 1u32 - 1) ;
             ({
                 InnerComponent_focusborder_17 :: FIELD_OFFSETS . r#focusborder_17 }
             + {
                 * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (4f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_focusborder_17 :: FIELD_OFFSETS . r#focusborder_17 }
                 + {
                     * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerComponent_focusborder_17 :: FIELD_OFFSETS . r#focusborder_17 }
                 + {
                     * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerComponent_focusborder_17 :: FIELD_OFFSETS . r#focusborder_17 }
             + {
                 * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_17 :: FIELD_OFFSETS . r#focusborder_17 }
             + {
                 * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerComponent_focusborder_17 :: FIELD_OFFSETS . r#focusborder_17 }
             + {
                 * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerFocusBorder_root_5 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#focusborder_17 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     InnerComponent_focusborder_17 :: FIELD_OFFSETS . r#focusborder_17 }
                 + {
                     * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_17 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_17 . tree_index . get ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     InnerComponent_focusborder_17 :: FIELD_OFFSETS . r#focusborder_17 }
                 + {
                     * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . r#focusborder_17 . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . r#focusborder_17 . tree_index . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => ((_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , (_self . parent . upgrade () . as_ref () . map (| x | (InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 ..= 1u32 => return {
                     * & Self :: FIELD_OFFSETS . r#focusborder_17 }
                 . apply_pin (_self) . item_geometry (index - 1u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_17 }
                 . apply_pin (_self) . accessible_role (0) , 1u32 ..= 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_17 }
                 . apply_pin (_self) . accessible_role (index - 1u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_17 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (1u32 ..= 1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_17 }
                 . apply_pin (_self) . accessible_string_property (index - 1u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (0u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_17 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (1u32 ..= 1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_17 }
                 . apply_pin (_self) . accessibility_action (index - 1u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_17 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 1u32 ..= 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_17 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 1u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 ..= 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#focusborder_17 }
                 . apply_pin (_self) . item_element_infos (index - 1u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_focusborder_17 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_7 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerButton_root_7 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_focusborder_17 , sp :: ItemVTable , sp :: AllowPin > ;
             2usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 InnerComponent_focusborder_17 :: FIELD_OFFSETS . r#focusborder_17 }
             + {
                 * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#root_5 }
            ) , sp :: VOffset :: new ({
                 InnerComponent_focusborder_17 :: FIELD_OFFSETS . r#focusborder_17 }
             + {
                 * & InnerFocusBorder_root_5 :: FIELD_OFFSETS . r#rectangle_6 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_focusborder_17) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_focusborder_17 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_focusborder_17 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_focusborder_17 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_focusborder_17 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 4u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_focusborder_17 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_focusborder_17 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] pub struct InnerStandardButton_root_19 {
         r#root_19 : sp :: r#Empty , r#base_21 : InnerButton_root_7 , r#root_19_empty_20_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_19_empty_20_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_19_empty_20_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_19_height : sp :: Property < sp :: LogicalLength > , r#root_19_kind : sp :: Property < sp :: r#StandardButtonKind > , r#root_19_width : sp :: Property < sp :: LogicalLength > , r#root_19_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerStandardButton_root_19 >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerStandardButton_root_19 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerButton_root_7 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#base_21 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 1u32 - 1 , tree_index_of_first_child + 2u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_empty_20_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                                 + {
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                         InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                                     + {
                                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_empty_20_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                             + {
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                                 + {
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_empty_20_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                             + {
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                                 + {
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_19_kind = ({
                             * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_kind }
                        ) . apply_pin (_self) . get () ;
                         if ((r#tmp_root_19_kind . clone ())) == ((sp :: r#StandardButtonKind :: r#Ok)) {
                             (sp :: SharedString :: from ("OK")) as _ }
                         else {
                             if ((r#tmp_root_19_kind . clone ())) == ((sp :: r#StandardButtonKind :: r#Cancel)) {
                                 (sp :: SharedString :: from ("Cancel")) as _ }
                             else {
                                 if ((r#tmp_root_19_kind . clone ())) == ((sp :: r#StandardButtonKind :: r#Apply)) {
                                     (sp :: SharedString :: from ("Apply")) as _ }
                                 else {
                                     if ((r#tmp_root_19_kind . clone ())) == ((sp :: r#StandardButtonKind :: r#Close)) {
                                         (sp :: SharedString :: from ("Close")) as _ }
                                     else {
                                         if ((r#tmp_root_19_kind . clone ())) == ((sp :: r#StandardButtonKind :: r#Reset)) {
                                             (sp :: SharedString :: from ("Reset")) as _ }
                                         else {
                                             if ((r#tmp_root_19_kind . clone ())) == ((sp :: r#StandardButtonKind :: r#Help)) {
                                                 (sp :: SharedString :: from ("Help")) as _ }
                                             else {
                                                 if ((r#tmp_root_19_kind . clone ())) == ((sp :: r#StandardButtonKind :: r#Yes)) {
                                                     (sp :: SharedString :: from ("Yes")) as _ }
                                                 else {
                                                     if ((r#tmp_root_19_kind . clone ())) == ((sp :: r#StandardButtonKind :: r#No)) {
                                                         (sp :: SharedString :: from ("No")) as _ }
                                                     else {
                                                         if ((r#tmp_root_19_kind . clone ())) == ((sp :: r#StandardButtonKind :: r#Abort)) {
                                                             (sp :: SharedString :: from ("Abort")) as _ }
                                                         else {
                                                             if ((r#tmp_root_19_kind . clone ())) == ((sp :: r#StandardButtonKind :: r#Retry)) {
                                                                 (sp :: SharedString :: from ("Retry")) as _ }
                                                             else {
                                                                 if ((r#tmp_root_19_kind . clone ())) == ((sp :: r#StandardButtonKind :: r#Ignore)) {
                                                                     (sp :: SharedString :: from ("Ignore")) as _ }
                                                                 else {
                                                                     sp :: SharedString :: from ("") }
                                                                 }
                                                             }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_empty_20_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_empty_20_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon_size }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_primary }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_y }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerButton_root_7 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#base_21 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#base_21 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) + ((({
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_empty_20_layoutinfo_h }
                ) . apply_pin (_self) . get ())) , sp :: Orientation :: Vertical => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                )) + ((({
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_empty_20_layoutinfo_v }
                ) . apply_pin (_self) . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#base_21 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#base_21 }
                     . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , ({
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_empty_20_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_empty_20_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 ..= 8u32 => return {
                     * & Self :: FIELD_OFFSETS . r#base_21 }
                 . apply_pin (_self) . item_geometry (index - 2u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Button , 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_21 }
                 . apply_pin (_self) . accessible_role (0) , 2u32 ..= 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_21 }
                 . apply_pin (_self) . accessible_role (index - 2u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (1u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (1u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (1u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text }
                ) . apply_pin (_self) . get ()) , (1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_21 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (2u32 ..= 8u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_21 }
                 . apply_pin (_self) . accessible_string_property (index - 2u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (1u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                     + {
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (1u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_21 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (2u32 ..= 8u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#base_21 }
                 . apply_pin (_self) . accessibility_action (index - 2u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: SupportedAccessibilityAction :: r#Default , 1u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_21 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 2u32 ..= 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_21 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 2u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 ..= 8u32 => {
                     * & Self :: FIELD_OFFSETS . r#base_21 }
                 . apply_pin (_self) . item_element_infos (index - 2u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerAboutDialog {
         r#root_22 : sp :: r#WindowItem , r#empty_23 : sp :: r#Empty , r#aboutslint_24 : InnerAboutSlint_root_1 , r#standardbutton_25 : InnerStandardButton_root_19 , r#root_22_empty_23_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_22_empty_23_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_22_empty_23_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_22_layout_cache_h : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_22_layout_cache_v : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_22_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_22_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_22_close : sp :: Callback < () , () > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerAboutDialog >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerAboutDialog {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             InnerAboutSlint_root_1 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#aboutslint_24 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 2u32 - 1 , tree_index_of_first_child + 4u32 - 1) ;
             InnerStandardButton_root_19 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 3u32 - 1 , tree_index_of_first_child + 6u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerFluentPalette_70 :: FIELD_OFFSETS . r#background . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerAboutDialog :: FIELD_OFFSETS . r#aboutslint_24 }
                                 + {
                                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (({
                                         InnerAboutDialog :: FIELD_OFFSETS . r#aboutslint_24 }
                                     + {
                                         * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layoutinfo_v }
                                    ) . apply_pin (_self) . get ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                )) + ((({
                                     InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
                                 + {
                                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_empty_20_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layout_cache_v }
                        ) . apply_pin (_self) . get () [1usize] as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAboutDialog :: FIELD_OFFSETS . r#aboutslint_24 }
                             + {
                                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (({
                                     InnerAboutDialog :: FIELD_OFFSETS . r#aboutslint_24 }
                                 + {
                                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ((({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        )) + ((({
                             InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
                         + {
                             * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_empty_20_layoutinfo_h }
                        ) . apply_pin (_self) . get ()))) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerAboutDialog :: FIELD_OFFSETS . r#aboutslint_24 }
                             + {
                                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (({
                                     InnerAboutDialog :: FIELD_OFFSETS . r#aboutslint_24 }
                                 + {
                                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_i_layout_2_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )) + ((({
                                 InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
                             + {
                                 * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_empty_20_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layout_cache_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#cells = [{
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layoutinfo_h }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ] ;
                         sp :: reorder_dialog_button_layout (& mut r#cells , & sp :: Slice :: from_slice (& [])) ;
                         let r#cells = sp :: Slice :: from_slice (& r#cells) ;
                         ;
                         sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                             r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 8f64 as _ ;
                                 the_struct . r#end = 8f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
                             + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as _ , r#spacing : 8f64 as _ , }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layout_cache_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_grid_layout (& sp :: GridLayoutData {
                         r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: GridLayoutCellData :: default () ;
                             the_struct . r#col_or_row = 0f64 as _ ;
                             the_struct . r#constraint = ({
                                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct . r#span = 1f64 as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 8f64 as _ , }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#grid_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: GridLayoutCellData :: default () ;
                         the_struct . r#col_or_row = 0f64 as _ ;
                         the_struct . r#constraint = ({
                             * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct . r#span = 1f64 as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("About Slint")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAboutDialog :: FIELD_OFFSETS . r#aboutslint_24 }
                 + {
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAboutDialog :: FIELD_OFFSETS . r#aboutslint_24 }
                 + {
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layout_cache_h }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAboutDialog :: FIELD_OFFSETS . r#aboutslint_24 }
                 + {
                     * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
                 + {
                     InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_close }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
                 + {
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
             + {
                 * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_kind }
            ) . apply_pin (_self) . set ({
                 (sp :: r#StandardButtonKind :: r#Close) as sp :: r#StandardButtonKind }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
                 + {
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layout_cache_h }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
                 + {
                     * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#resize_border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
             + {
                 * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19_kind }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerAboutSlint_root_1 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#aboutslint_24 }
             . apply_pin (x)) ,) ;
             InnerStandardButton_root_19 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
             . apply_pin (x)) ,) ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 0u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
                     . apply_pin (_self) . subtree_range (dyn_index - 0u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 ..= 2u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
                     . apply_pin (_self) . subtree_component (dyn_index - 0u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => (({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layout_cache_v }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layout_cache_h }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layout_cache_h }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layout_cache_v }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layout_cache_h }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , 0f64 as sp :: Coord , ({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 3u32 => (({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_layout_cache_h }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , 0f64 as sp :: Coord , ({
                     * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_empty_23_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 4u32 ..= 5u32 => return {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_24 }
                 . apply_pin (_self) . item_geometry (index - 4u32 + 1) , 6u32 ..= 13u32 => return {
                     * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
                 . apply_pin (_self) . item_geometry (index - 6u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 => {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_24 }
                 . apply_pin (_self) . accessible_role (0) , 4u32 ..= 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_24 }
                 . apply_pin (_self) . accessible_role (index - 4u32 + 1) , 3u32 => {
                     * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
                 . apply_pin (_self) . accessible_role (0) , 6u32 ..= 13u32 => {
                     * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
                 . apply_pin (_self) . accessible_role (index - 6u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (2u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_24 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (4u32 ..= 5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_24 }
                 . apply_pin (_self) . accessible_string_property (index - 4u32 + 1 , what) , (3u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (6u32 ..= 13u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
                 . apply_pin (_self) . accessible_string_property (index - 6u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (2u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_24 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (4u32 ..= 5u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_24 }
                 . apply_pin (_self) . accessibility_action (index - 4u32 + 1 , action) , (3u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (6u32 ..= 13u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
                 . apply_pin (_self) . accessibility_action (index - 6u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 2u32 => {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_24 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 4u32 ..= 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_24 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 4u32 + 1) , 3u32 => {
                     * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 6u32 ..= 13u32 => {
                     * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 6u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 4u32 ..= 5u32 => {
                     * & Self :: FIELD_OFFSETS . r#aboutslint_24 }
                 . apply_pin (_self) . item_element_infos (index - 4u32 + 1) , 6u32 ..= 13u32 => {
                     * & Self :: FIELD_OFFSETS . r#standardbutton_25 }
                 . apply_pin (_self) . item_element_infos (index - 6u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerAboutDialog {
         fn new () -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = SharedGlobals :: new (sp :: VRc :: downgrade (& self_dyn_rc)) ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             14usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 4u32 , parent_index : 1u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 6u32 , parent_index : 1u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 6u32 , parent_index : 2u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 6u32 , parent_index : 2u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 7u32 , parent_index : 3u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 11u32 , parent_index : 6u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 6u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 6u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 7u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 7u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 7u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerAboutDialog , sp :: ItemVTable , sp :: AllowPin > ;
             11usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22 }
            ) , sp :: VOffset :: new ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#empty_23 }
            ) , sp :: VOffset :: new ({
                 InnerAboutDialog :: FIELD_OFFSETS . r#aboutslint_24 }
             + {
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
             + {
                 * & InnerStandardButton_root_19 :: FIELD_OFFSETS . r#root_19 }
            ) , sp :: VOffset :: new ({
                 InnerAboutDialog :: FIELD_OFFSETS . r#aboutslint_24 }
             + {
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#image_3 }
            ) , sp :: VOffset :: new ({
                 InnerAboutDialog :: FIELD_OFFSETS . r#aboutslint_24 }
             + {
                 * & InnerAboutSlint_root_1 :: FIELD_OFFSETS . r#text_4 }
            ) , sp :: VOffset :: new ({
                 InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
             + {
                 InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7 }
            ) , sp :: VOffset :: new ({
                 InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
             + {
                 InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_background_8 }
            ) , sp :: VOffset :: new ({
                 InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
             + {
                 InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_touch_area_15 }
            ) , sp :: VOffset :: new ({
                 InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
             + {
                 InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
            ) , sp :: VOffset :: new ({
                 InnerAboutDialog :: FIELD_OFFSETS . r#standardbutton_25 }
             + {
                 InnerStandardButton_root_19 :: FIELD_OFFSETS . r#base_21 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_border_9 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerAboutDialog) ;
         }
     ;
     impl sp :: PinnedDrop for InnerAboutDialog {
         fn drop (self : :: core :: pin :: Pin < & mut InnerAboutDialog >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerAboutDialog {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerAboutDialog > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#AboutDialog (sp :: VRc < sp :: ItemTreeVTable , InnerAboutDialog >) ;
     impl r#AboutDialog {
         pub fn new () -> :: core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerAboutDialog :: new () ? ;
             inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             InnerAboutDialog :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn invoke_close (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_close }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_close (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerAboutDialog :: FIELD_OFFSETS . r#root_22_close }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         }
     impl From < r#AboutDialog > for sp :: VRc < sp :: ItemTreeVTable , InnerAboutDialog > {
         fn from (value : r#AboutDialog) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#AboutDialog {
         type WeakInner = sp :: VWeak < sp :: ItemTreeVTable , InnerAboutDialog > ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (sp :: VRc :: downgrade (& self . 0)) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn upgrade_from_weak_inner (inner : & Self :: WeakInner) -> sp :: Option < Self > {
             sp :: Some (Self (inner . upgrade () ?)) }
         fn run (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             :: core :: result :: Result :: Ok (()) }
         fn show (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerMainWindow {
         r#root_26 : sp :: r#WindowItem , r#scroll_28 : sp :: r#Empty , r#flickable_29 : sp :: r#Flickable , r#flickable_viewport_30 : sp :: r#Empty , r#vertical_bar_visibility_38 : sp :: r#Clip , r#vertical_bar_39 : sp :: r#BasicBorderRectangle , r#vertical_bar_clip_40 : sp :: r#Clip , r#thumb_41 : sp :: r#BasicBorderRectangle , r#touch_area_42 : sp :: r#TouchArea , r#up_scroll_button_Opacity_43 : sp :: r#Opacity , r#up_scroll_button_44 : sp :: r#TouchArea , r#icon_Opacity_45 : sp :: r#Opacity , r#icon_46 : sp :: r#ImageItem , r#down_scroll_button_Opacity_47 : sp :: r#Opacity , r#down_scroll_button_48 : sp :: r#TouchArea , r#icon_Opacity_49 : sp :: r#Opacity , r#icon_50 : sp :: r#ImageItem , r#horizontal_bar_visibility_51 : sp :: r#Clip , r#horizontal_bar_52 : sp :: r#BasicBorderRectangle , r#horizontal_bar_clip_53 : sp :: r#Clip , r#thumb_54 : sp :: r#BasicBorderRectangle , r#touch_area_55 : sp :: r#TouchArea , r#up_scroll_button_Opacity_56 : sp :: r#Opacity , r#up_scroll_button_57 : sp :: r#TouchArea , r#icon_Opacity_58 : sp :: r#Opacity , r#icon_59 : sp :: r#ImageItem , r#down_scroll_button_Opacity_60 : sp :: r#Opacity , r#down_scroll_button_61 : sp :: r#TouchArea , r#icon_Opacity_62 : sp :: r#Opacity , r#icon_63 : sp :: r#ImageItem , r#empty_64 : sp :: r#Empty , r#text_65 : sp :: r#SimpleText , r#button_66 : InnerButton_root_7 , r#button_67 : InnerButton_root_7 , r#button_68 : InnerButton_root_7 , r#root_26_down_scroll_button_48_state : sp :: Property < i32 > , r#root_26_down_scroll_button_61_state : sp :: Property < i32 > , r#root_26_empty_27_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_26_empty_27_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_26_empty_27_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_26_empty_31_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_26_empty_31_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_26_empty_31_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_26_empty_31_width : sp :: Property < sp :: LogicalLength > , r#root_26_empty_64_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_26_empty_64_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_26_empty_64_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_26_flickable_29_height : sp :: Property < sp :: LogicalLength > , r#root_26_flickable_29_horizontal_stretch : sp :: Property < f32 > , r#root_26_flickable_29_max_height : sp :: Property < sp :: LogicalLength > , r#root_26_flickable_29_max_width : sp :: Property < sp :: LogicalLength > , r#root_26_flickable_29_min_height : sp :: Property < sp :: LogicalLength > , r#root_26_flickable_29_min_width : sp :: Property < sp :: LogicalLength > , r#root_26_flickable_29_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_26_flickable_29_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_26_flickable_29_vertical_stretch : sp :: Property < f32 > , r#root_26_flickable_29_width : sp :: Property < sp :: LogicalLength > , r#root_26_grid : sp :: Property < sp :: ModelRc < sp :: ModelRc < r#Tile > > > , r#root_26_horizontal_bar_52_maximum : sp :: Property < sp :: LogicalLength > , r#root_26_horizontal_bar_52_policy : sp :: Property < sp :: r#ScrollBarPolicy > , r#root_26_horizontal_bar_52_size : sp :: Property < sp :: LogicalLength > , r#root_26_horizontal_bar_52_state : sp :: Property < i32 > , r#root_26_horizontal_bar_52_visible : sp :: Property < bool > , r#root_26_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_26_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_26_mine_value : sp :: Property < i32 > , r#root_26_scroll_28_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_26_scroll_28_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_26_scroll_28_vertical_scrollbar_policy : sp :: Property < sp :: r#ScrollBarPolicy > , r#root_26_state : sp :: Property < r#GameState > , r#root_26_thumb_41_height : sp :: Property < sp :: LogicalLength > , r#root_26_thumb_41_width : sp :: Property < sp :: LogicalLength > , r#root_26_thumb_41_y : sp :: Property < sp :: LogicalLength > , r#root_26_thumb_54_height : sp :: Property < sp :: LogicalLength > , r#root_26_thumb_54_width : sp :: Property < sp :: LogicalLength > , r#root_26_thumb_54_x : sp :: Property < sp :: LogicalLength > , r#root_26_touch_area_42_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_26_touch_area_42_saved_maximum : sp :: Property < sp :: LogicalLength > , r#root_26_touch_area_42_saved_x : sp :: Property < sp :: LogicalLength > , r#root_26_touch_area_42_saved_y : sp :: Property < sp :: LogicalLength > , r#root_26_touch_area_55_pressed_value : sp :: Property < sp :: LogicalLength > , r#root_26_touch_area_55_saved_maximum : sp :: Property < sp :: LogicalLength > , r#root_26_touch_area_55_saved_x : sp :: Property < sp :: LogicalLength > , r#root_26_touch_area_55_saved_y : sp :: Property < sp :: LogicalLength > , r#root_26_up_scroll_button_44_state : sp :: Property < i32 > , r#root_26_up_scroll_button_57_state : sp :: Property < i32 > , r#root_26_vertical_bar_39_maximum : sp :: Property < sp :: LogicalLength > , r#root_26_vertical_bar_39_size : sp :: Property < sp :: LogicalLength > , r#root_26_vertical_bar_39_state : sp :: Property < i32 > , r#root_26_vertical_bar_39_visible : sp :: Property < bool > , r#root_26_about : sp :: Callback < () , () > , r#root_26_change_flag : sp :: Callback < (r#Position , bool ,) , () > , r#root_26_change_visibility : sp :: Callback < (r#Position , bool ,) , () > , r#root_26_check_win : sp :: Callback < () , () > , r#root_26_close : sp :: Callback < () , () > , r#root_26_expand_selection : sp :: Callback < (r#Position ,) , () > , r#root_26_first_move_occured : sp :: Callback < (r#Position ,) , () > , r#root_26_horizontal_bar_52_scrolled : sp :: Callback < () , () > , r#root_26_restart : sp :: Callback < () , () > , r#root_26_vertical_bar_39_scrolled : sp :: Callback < () , () > , repeater0 : sp :: Repeater < InnerComponent_empty_32 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerMainWindow >> , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerMainWindow {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_grid }
                    ) . apply_pin (_self) . get ()) as _ }
                 }
            ) ;
             InnerButton_root_7 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_66 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 33u32 - 1 , tree_index_of_first_child + 36u32 - 1) ;
             InnerButton_root_7 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_67 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 34u32 - 1 , tree_index_of_first_child + 43u32 - 1) ;
             InnerButton_root_7 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_68 }
             . apply_pin (x)) , _self . globals . get () . unwrap () . clone () , tree_index_of_first_child + 35u32 - 1 , tree_index_of_first_child + 50u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerFluentPalette_70 :: FIELD_OFFSETS . r#background . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (28f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_down_scroll_button_48_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_48 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_48 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_down_scroll_button_61_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_61 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_61 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_scroll_28_layoutinfo_v }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = 50f64 as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 8f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_scroll_28_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_scroll_28_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = 50f64 as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_31_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_empty_32 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 0f64 as _ ;
                                 the_struct . r#end = 0f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as _ , r#spacing : 0f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_31_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_empty_32 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_31_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_empty_32 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 0f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_31_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
                            ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 32u32 - 1)) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                                 + {
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                         InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                                     + {
                                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                                 + {
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                         InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                                     + {
                                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                                 + {
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                         InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                                     + {
                                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_h }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 10f64 as _ ;
                             the_struct . r#end = 10f64 as _ ;
                             the_struct }
                         as _ , r#size : (((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((8f64) as f64)) as f64) - ((8f64) as f64)) as _ , r#spacing : 10f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 32u32 - 1)) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                             + {
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                                 + {
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                             + {
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                                 + {
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                             + {
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                                 + {
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 10f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 10f64 as _ ;
                         the_struct . r#end = 10f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 32u32 - 1)) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                             + {
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                                 + {
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                             + {
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                                 + {
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                             + {
                                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max ((({
                                     InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                                 + {
                                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_i_layout_10_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 10f64 as _ ;
                         the_struct . r#end = 10f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 3u32 - 1))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((8f64) as f64)) as f64) - ((8f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get ()) as f64) - ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_policy }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#ScrollBarPolicy :: r#AsNeeded) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_scrolled }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#flicked) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (6f64) as _ }
                     else {
                         2f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_55 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) || ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_61 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())))) || ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_57 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_26_horizontal_bar_52_policy = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_policy }
                        ) . apply_pin (_self) . get () ;
                         ((((r#tmp_root_26_horizontal_bar_52_policy . clone ())) == ((sp :: r#ScrollBarPolicy :: r#AlwaysOn)))) || ((((((r#tmp_root_26_horizontal_bar_52_policy . clone ())) == ((sp :: r#ScrollBarPolicy :: r#AsNeeded)))) && ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_maximum }
                        ) . apply_pin (_self) . get () . get ()) as f64) > ((0f64) as f64))))) }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_5 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_4 = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_4 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_4 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (r#layout_info_4 . clone ()) . r#min as _ ;
                             the_struct . r#min_percent = (r#layout_info_4 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = 1280f64 as _ ;
                             the_struct . r#stretch = (r#layout_info_4 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    )) + ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layoutinfo_h }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#layout_info_5 = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index . get ())) ;
                         {
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = (r#layout_info_5 . clone ()) . r#max as _ ;
                             the_struct . r#max_percent = (r#layout_info_5 . clone ()) . r#max_percent as _ ;
                             the_struct . r#min = (r#layout_info_5 . clone ()) . r#min as _ ;
                             the_struct . r#min_percent = (r#layout_info_5 . clone ()) . r#min_percent as _ ;
                             the_struct . r#preferred = 720f64 as _ ;
                             the_struct . r#stretch = (r#layout_info_5 . clone ()) . r#stretch as _ ;
                             the_struct }
                         }
                    )) + ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layoutinfo_v }
                    ) . apply_pin (_self) . get ())))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_scroll_28_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    )))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_scroll_28_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    )))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_scroll_28_vertical_scrollbar_policy }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#ScrollBarPolicy :: r#AsNeeded) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_state }
            ) . apply_pin (_self) . set ({
                 (r#GameState :: r#Initial) as r#GameState }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_41_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_root_26_vertical_bar_39_maximum = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_maximum }
                        ) . apply_pin (_self) . get () . get () ;
                         ((if ((r#tmp_root_26_vertical_bar_39_maximum . clone ()) as f64) <= ((((0f64) as f64) / ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64)) as f64) {
                             (0f64) as _ }
                         else {
                             {
                                 let r#tmp_root_26_vertical_bar_39_page_size = ({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                                ) . apply_pin (_self) . get () . get () ;
                                 ((((16f64 as sp :: Coord) . min ((((({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord) as sp :: Coord) . max ((((((((({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) * ((((r#tmp_root_26_vertical_bar_39_page_size . clone ()) as f64) / ((((r#tmp_root_26_vertical_bar_39_maximum . clone ()) as f64) + ((r#tmp_root_26_vertical_bar_39_page_size . clone ()) as f64)) as f64)) as f64)) as sp :: Coord)) as f64) * ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64) }
                             }
                        ) as f64) / ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64) }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_41_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_size }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_41_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((16f64) as f64) + ((((((((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                    ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) - ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_41_height }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((- ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) / ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_maximum }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_54_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_size }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_54_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_root_26_horizontal_bar_52_maximum = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_maximum }
                        ) . apply_pin (_self) . get () . get () ;
                         ((if ((r#tmp_root_26_horizontal_bar_52_maximum . clone ()) as f64) <= ((((0f64) as f64) / ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64)) as f64) {
                             (0f64) as _ }
                         else {
                             {
                                 let r#tmp_root_26_horizontal_bar_52_page_size = ({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                                ) . apply_pin (_self) . get () . get () ;
                                 ((((16f64 as sp :: Coord) . min ((((({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord) as sp :: Coord) . max ((((((((((({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) * ((r#tmp_root_26_horizontal_bar_52_page_size . clone ()) as f64)) as f64) / ((((r#tmp_root_26_horizontal_bar_52_maximum . clone ()) as f64) + ((r#tmp_root_26_horizontal_bar_52_page_size . clone ()) as f64)) as f64)) as sp :: Coord)) as f64) * ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64) }
                             }
                        ) as f64) / ((sp :: WindowInner :: from_pub (& _self . globals . get () . unwrap () . window_adapter_impl () . window ()) . scale_factor ()) as f64) }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_54_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((16f64) as f64) + ((((((((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                    ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) - ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_54_width }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((- ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) / ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_maximum }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("MineSweeper")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_up_scroll_button_44_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_44 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_44 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_up_scroll_button_57_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_57 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         (1f64) as _ }
                     else {
                         if ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_57 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             (2f64) as _ }
                         else {
                             0f64 }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_maximum }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get ()) as f64) - ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                    ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_scrolled }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#flicked) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_size }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (6f64) as _ }
                     else {
                         2f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_42 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) || ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_48 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())))) || ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_44 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get ())) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_visible }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_26_scroll_28_vertical_scrollbar_policy = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_scroll_28_vertical_scrollbar_policy }
                        ) . apply_pin (_self) . get () ;
                         ((((r#tmp_root_26_scroll_28_vertical_scrollbar_policy . clone ())) == ((sp :: r#ScrollBarPolicy :: r#AlwaysOn)))) || ((((((r#tmp_root_26_scroll_28_vertical_scrollbar_policy . clone ())) == ((sp :: r#ScrollBarPolicy :: r#AsNeeded)))) && ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_maximum }
                        ) . apply_pin (_self) . get () . get ()) as f64) > ((0f64) as f64))))) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#interactive) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_31_layoutinfo_v }
                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_31_layoutinfo_h }
                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (0f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_38 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_visible }
                    ) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_39 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4281084972f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((4293980400f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_39 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_39 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_clip_40 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_clip_40 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_clip_40 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_clip_40 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_clip_40 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_clip_40 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#thumb_41 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#thumb_41 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_41_width }
                    ) . apply_pin (_self) . get () . get ()) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_42 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_42 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true)) && ((({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_42 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 ({
                                     if ! sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_42_saved_maximum }
                                    ) . apply_pin (_self) . get () . get () as f64) , & (({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_maximum }
                                    ) . apply_pin (_self) . get () . get () as f64)) {
                                         ({
                                             _self . r#fn_touch_area_42_update_saved_values () }
                                        ) ;
                                         }
                                     else {
                                         {
                                             }
                                         }
                                     ;
                                     ({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- (0f64 as sp :: Coord) . max ((({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_42_pressed_value }
                                    ) . apply_pin (_self) . get () . get ()) as f64) + ((if false {
                                         ((((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_42 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get ()) as f64) - ((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_42_saved_x }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as f64) / ((((((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) - ((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_41_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64))) as _ }
                                     else {
                                         ((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_42 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get ()) as f64) - ((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_42_saved_y }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as f64) / ((((((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) - ((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_41_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64) }
                                    ) as f64)) as sp :: Coord) as sp :: Coord)) as sp :: Coord) as _) ;
                                     ({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_scrolled }
                                    ) . apply_pin (_self) . call (& ()) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_42 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button)) == ((sp :: r#PointerEventButton :: r#Left)))) && (((((args . 0 . clone ()) . r#kind)) == ((sp :: r#PointerEventKind :: r#Down)))) {
                                 ({
                                     _self . r#fn_touch_area_42_update_saved_values () }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_42 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression0 = {
                                 let r#return_check_merge0 = if ((false)) && ((! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_x as f64) , & (0f64 as f64)))) {
                                     ((false , {
                                         ({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((0f64 as sp :: Coord) . min ((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) + (((args . 0 . clone ()) . r#delta_x) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,)) as _ }
                                 else {
                                     if ! (((! false)) && ((! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_y as f64) , & (0f64 as f64))))) {
                                         ((true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                     else {
                                         (false , {
                                             ({
                                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_maximum }
                                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) + (((args . 0 . clone ()) . r#delta_y) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,) }
                                     }
                                 ;
                                 if (r#return_check_merge0 . clone ()) . 0 {
                                     (({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                 else {
                                     (sp :: r#EventResult :: r#Reject , false , (r#return_check_merge0 . clone ()) . 1 ,) }
                                 }
                             ;
                             if (r#returned_expression0 . clone ()) . 1 {
                                 ((r#returned_expression0 . clone ()) . 0) as _ }
                             else {
                                 (r#returned_expression0 . clone ()) . 2 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_Opacity_43 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_44 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min ((((({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) + ((10f64) as f64)) as sp :: Coord) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_44 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_Opacity_45 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_up_scroll_button_44_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        ) }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_18 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         (((r#image_implicit_size . clone ()) . r#height) as f64) / (((r#image_implicit_size . clone ()) . r#width) as f64) }
                    ) as f64) * ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_18 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_up_scroll_button_44_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (6f64) as _ }
                     else {
                         8f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_Opacity_47 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_48 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_maximum }
                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((((({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as sp :: Coord) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_48 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_Opacity_49 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_down_scroll_button_48_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        ) }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         (((r#image_implicit_size . clone ()) . r#height) as f64) / (((r#image_implicit_size . clone ()) . r#width) as f64) }
                    ) as f64) * ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_down_scroll_button_48_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (6f64) as _ }
                     else {
                         8f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_51 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((! ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_visible }
                    ) . apply_pin (_self) . get ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_52 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((4281084972f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((4293980400f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded ((0f64) as u32)) }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_52 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_52 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_53 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_53 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_53 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_53 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (7f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_53 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_53 }
                 + sp :: r#Clip :: FIELD_OFFSETS . r#clip) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#thumb_54 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                         (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                     else {
                         sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#thumb_54 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_54_height }
                    ) . apply_pin (_self) . get () . get ()) as f64) / ((2f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_55 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_55 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#moved) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ((true)) && ((({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_55 }
                             + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get ())) {
                                 ({
                                     if ! sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_55_saved_maximum }
                                    ) . apply_pin (_self) . get () . get () as f64) , & (({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_maximum }
                                    ) . apply_pin (_self) . get () . get () as f64)) {
                                         ({
                                             _self . r#fn_touch_area_55_update_saved_values () }
                                        ) ;
                                         }
                                     else {
                                         {
                                             }
                                         }
                                     ;
                                     ({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                                     + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- (0f64 as sp :: Coord) . max ((({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_maximum }
                                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . min ((((({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_55_pressed_value }
                                    ) . apply_pin (_self) . get () . get ()) as f64) + ((if true {
                                         ((((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_55 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get ()) as f64) - ((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_55_saved_x }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as f64) / ((((((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) - ((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_54_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64))) as _ }
                                     else {
                                         ((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_55 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get ()) as f64) - ((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_55_saved_y }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64) * ((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as f64) / ((((((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                                        ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((32f64) as f64)) as f64) - ((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_54_height }
                                        ) . apply_pin (_self) . get () . get ()) as f64)) as f64)) as f64) }
                                    ) as f64)) as sp :: Coord) as sp :: Coord)) as sp :: Coord) as _) ;
                                     ({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_scrolled }
                                    ) . apply_pin (_self) . call (& ()) }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_55 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button)) == ((sp :: r#PointerEventButton :: r#Left)))) && (((((args . 0 . clone ()) . r#kind)) == ((sp :: r#PointerEventKind :: r#Down)))) {
                                 ({
                                     _self . r#fn_touch_area_55_update_saved_values () }
                                ) ;
                                 }
                             else {
                                 {
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_55 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#scroll_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression1 = {
                                 let r#return_check_merge1 = if ((true)) && ((! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_x as f64) , & (0f64 as f64)))) {
                                     ((false , {
                                         ({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_maximum }
                                        ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((0f64 as sp :: Coord) . min ((((({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                                         + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) + (((args . 0 . clone ()) . r#delta_x) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,)) as _ }
                                 else {
                                     if ! (((! true)) && ((! sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#delta_y as f64) , & (0f64 as f64))))) {
                                         ((true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                     else {
                                         (false , {
                                             ({
                                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_maximum }
                                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((0f64 as sp :: Coord) . min ((((({
                                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) + (((args . 0 . clone ()) . r#delta_y) as f64)) as sp :: Coord) as sp :: Coord) as sp :: Coord) as _) ;
                                             sp :: r#EventResult :: r#Accept }
                                         ,) }
                                     }
                                 ;
                                 if (r#return_check_merge1 . clone ()) . 0 {
                                     (({
                                         sp :: r#EventResult :: r#Reject }
                                     , true , sp :: r#EventResult :: r#Reject ,)) as _ }
                                 else {
                                     (sp :: r#EventResult :: r#Reject , false , (r#return_check_merge1 . clone ()) . 1 ,) }
                                 }
                             ;
                             if (r#returned_expression1 . clone ()) . 1 {
                                 ((r#returned_expression1 . clone ()) . 0) as _ }
                             else {
                                 (r#returned_expression1 . clone ()) . 2 }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_Opacity_56 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_57 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new ((0f64 as sp :: Coord) . min ((((({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) + ((10f64) as f64)) as sp :: Coord) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_57 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_Opacity_58 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_up_scroll_button_57_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        ) }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_19 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         (((r#image_implicit_size . clone ()) . r#height) as f64) / (((r#image_implicit_size . clone ()) . r#width) as f64) }
                    ) as f64) * ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_19 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_up_scroll_button_57_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (4f64) as _ }
                     else {
                         6f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_Opacity_60 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (1f64) as _ }
                     else {
                         0f64 }
                    ) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_61 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . set (sp :: LogicalLength :: new (((- ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_maximum }
                            ) . apply_pin (_self) . get () . get ()) as sp :: Coord) . max ((((({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                             + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as f64) - ((10f64) as f64)) as sp :: Coord) as sp :: Coord) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_61 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_Opacity_62 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (1f64) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_down_scroll_button_61_state }
                    ) . apply_pin (_self) . get () as f64) , & (2f64 as f64)) {
                         (slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((3388997631f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((2566914048f64) as u32) }
                        )) as _ }
                     else {
                         slint :: Brush :: SolidColor (if InnerFluentPalette_70 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get () {
                             (sp :: Color :: from_argb_encoded ((352321535f64) as u32)) as _ }
                         else {
                             sp :: Color :: from_argb_encoded ((1929379840f64) as u32) }
                        ) }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         (((r#image_implicit_size . clone ()) . r#height) as f64) / (((r#image_implicit_size . clone ()) . r#width) as f64) }
                    ) as f64) * ((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
                     + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if sp :: ApproxEq :: < f64 > :: approx_eq (& (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_down_scroll_button_61_state }
                    ) . apply_pin (_self) . get () as f64) , & (1f64 as f64)) {
                         (4f64) as _ }
                     else {
                         6f64 }
                     as sp :: Coord)) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#direction = sp :: r#AnimationDirection :: r#Normal as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: CubicBezier ([0f32 , 0f32 , 0.58f32 , 1f32]) as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (InnerFluentPalette_70 :: FIELD_OFFSETS . r#foreground . apply_pin (_self . globals . get () . unwrap () . global_FluentPalette_70 . as_ref ()) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                    ) . apply_pin (_self) . get () [3usize]) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_root_26_state = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_state }
                        ) . apply_pin (_self) . get () ;
                         if ((r#tmp_root_26_state . clone ())) == ((r#GameState :: r#Lose)) {
                             (sp :: SharedString :: from ("State: Game Over")) as _ }
                         else {
                             if ((r#tmp_root_26_state . clone ())) == ((r#GameState :: r#Win)) {
                                 (sp :: SharedString :: from ("State: You Won")) as _ }
                             else {
                                 sp :: SharedString :: from ("State: Normal") }
                             }
                         }
                    ) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_restart }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                    ) . apply_pin (_self) . get () [3usize]) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon }
            ) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_20 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon_size }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (28f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Restart")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_about }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                    ) . apply_pin (_self) . get () [3usize]) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon }
            ) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon_size }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (28f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("About")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                    ) . apply_pin (_self) . get () [5usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                    ) . apply_pin (_self) . get () [4usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_close }
                            ) . apply_pin (_self) . call (& ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                    ) . apply_pin (_self) . get () [3usize]) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon }
            ) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_21 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon_size }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (28f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text }
            ) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Quit")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                    ) . apply_pin (_self) . get () [7usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                    ) . apply_pin (_self) . get () [6usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_y }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#resize_border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_38 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_38 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_38 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_38 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_38 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_39 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_39 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_39 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_clip_40 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_clip_40 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_clip_40 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_clip_40 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_clip_40 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#thumb_41 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#thumb_41 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_42 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_42 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_44 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_44 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_48 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_48 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_51 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_51 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_51 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_51 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_51 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_52 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_52 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_52 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_53 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_53 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_53 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_53 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_53 }
             + sp :: r#Clip :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#thumb_54 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#thumb_54 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_55 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_55 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_57 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_57 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_61 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_61 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
             + sp :: r#SimpleText :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon_size }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_primary }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon_size }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_primary }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_icon_size }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_primary }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_y }
            ) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             InnerButton_root_7 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_66 }
             . apply_pin (x)) ,) ;
             InnerButton_root_7 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_67 }
             . apply_pin (x)) ,) ;
             InnerButton_root_7 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#button_68 }
             . apply_pin (x)) ,) ;
             {
                 }
             ;
             {
                 {
                     {
                         }
                     ;
                     {
                         }
                     }
                 ;
                 {
                     {
                         }
                     ;
                     {
                         }
                     }
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_empty_32 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 ..= 3u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_66 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 1u32 , order , visitor) }
                 4u32 ..= 6u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_67 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 4u32 , order , visitor) }
                 7u32 ..= 9u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_68 }
                     . apply_pin (_self) . visit_dynamic_children (dyn_index - 7u32 , order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_layoutinfo_h }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = 1280f64 as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_layoutinfo_v }
                    ) . apply_pin (_self) . get () ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = (r#layout_info . clone ()) . r#min as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = 720f64 as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_empty_32 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 ..= 3u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_66 }
                     . apply_pin (_self) . subtree_range (dyn_index - 1u32) }
                 4u32 ..= 6u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_67 }
                     . apply_pin (_self) . subtree_range (dyn_index - 4u32) }
                 7u32 ..= 9u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_68 }
                     . apply_pin (_self) . subtree_range (dyn_index - 7u32) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_empty_32 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 ..= 3u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_66 }
                     . apply_pin (_self) . subtree_component (dyn_index - 1u32 , subtree_index , result) }
                 4u32 ..= 6u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_67 }
                     . apply_pin (_self) . subtree_component (dyn_index - 4u32 , subtree_index , result) }
                 7u32 ..= 9u32 => {
                     {
                         * & Self :: FIELD_OFFSETS . r#button_68 }
                     . apply_pin (_self) . subtree_component (dyn_index - 7u32 , subtree_index , result) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , (((((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((8f64) as f64)) as f64) - ((8f64) as f64)) as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , (((((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64) - ((8f64) as f64)) as f64) - ((8f64) as f64)) as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 3u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 4u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 5u32 => (0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 6u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 8u32 => ((((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord , 14f64 as sp :: Coord , (((((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                ) . apply_pin (_self) . get () . get ()) as f64) + ((0f64) as f64)) as f64) - ((14f64) as f64)) as sp :: Coord , 0f64 as sp :: Coord ,) , 9u32 => ((((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord , 14f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 10u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_41_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_41_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , (((10f64) as f64) - ((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_41_width }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_41_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 11u32 => ((((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord , 14f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 12u32 => (6f64 as sp :: Coord , 8f64 as sp :: Coord , 3f64 as sp :: Coord , 4f64 as sp :: Coord ,) , 13u32 => (6f64 as sp :: Coord , 8f64 as sp :: Coord , 3f64 as sp :: Coord , (((((((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((6f64) as f64)) as f64) - ((4f64) as f64)) as sp :: Coord ,) , 14u32 => (6f64 as sp :: Coord , 8f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 15u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (((((8f64) as f64) - ((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord , (((((6f64) as f64) - ((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , 16u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 17u32 => (6f64 as sp :: Coord , 8f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 18u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (((((8f64) as f64) - ((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord , (((((6f64) as f64) - ((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , 19u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 20u32 => (14f64 as sp :: Coord , (((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord , 0f64 as sp :: Coord , (((((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_height }
                ) . apply_pin (_self) . get () . get ()) as f64) + ((0f64) as f64)) as f64) - ((14f64) as f64)) as sp :: Coord ,) , 21u32 => (14f64 as sp :: Coord , (((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 22u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_54_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_54_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_54_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , (((10f64) as f64) - ((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_thumb_54_height }
                ) . apply_pin (_self) . get () . get ()) as f64)) as sp :: Coord ,) , 23u32 => (14f64 as sp :: Coord , (((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 24u32 => (8f64 as sp :: Coord , 6f64 as sp :: Coord , 4f64 as sp :: Coord , 3f64 as sp :: Coord ,) , 25u32 => (8f64 as sp :: Coord , 6f64 as sp :: Coord , (((((((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_flickable_29_width }
                ) . apply_pin (_self) . get () . get ()) as f64) - ((14f64) as f64)) as f64) - ((6f64) as f64)) as f64) - ((4f64) as f64)) as sp :: Coord , 3f64 as sp :: Coord ,) , 26u32 => (8f64 as sp :: Coord , 6f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 27u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (((((6f64) as f64) - ((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord , (((((8f64) as f64) - ((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , 28u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 29u32 => (8f64 as sp :: Coord , 6f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 30u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , (((((6f64) as f64) - ((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord , (((((8f64) as f64) - ((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get ()) as f64)) as f64) / ((2f64) as f64)) as sp :: Coord ,) , 31u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 32u32 => ((((((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                ) . apply_pin (_self) . get () [3usize]) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , 10f64 as sp :: Coord ,) , 33u32 => ((((((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                ) . apply_pin (_self) . get () [3usize]) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , 10f64 as sp :: Coord ,) , 34u32 => ((((((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                ) . apply_pin (_self) . get () [3usize]) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                ) . apply_pin (_self) . get () [5usize] as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , 10f64 as sp :: Coord ,) , 35u32 => ((((((({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_27_layout_cache }
                ) . apply_pin (_self) . get () [3usize]) as f64) - ((10f64) as f64)) as f64) - ((10f64) as f64)) as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                ) . apply_pin (_self) . get () [7usize] as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_64_layout_cache }
                ) . apply_pin (_self) . get () [6usize] as sp :: Coord , 10f64 as sp :: Coord ,) , 36u32 ..= 42u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_66 }
                 . apply_pin (_self) . item_geometry (index - 36u32 + 1) , 43u32 ..= 49u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_67 }
                 . apply_pin (_self) . item_geometry (index - 43u32 + 1) , 50u32 ..= 56u32 => return {
                     * & Self :: FIELD_OFFSETS . r#button_68 }
                 . apply_pin (_self) . item_geometry (index - 50u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 32u32 => sp :: r#AccessibleRole :: r#Text , 33u32 => sp :: r#AccessibleRole :: r#Button , 34u32 => sp :: r#AccessibleRole :: r#Button , 35u32 => sp :: r#AccessibleRole :: r#Button , 33u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_66 }
                 . apply_pin (_self) . accessible_role (0) , 36u32 ..= 42u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_66 }
                 . apply_pin (_self) . accessible_role (index - 36u32 + 1) , 34u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_67 }
                 . apply_pin (_self) . accessible_role (0) , 43u32 ..= 49u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_67 }
                 . apply_pin (_self) . accessible_role (index - 43u32 + 1) , 35u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_68 }
                 . apply_pin (_self) . accessible_role (0) , 50u32 ..= 56u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_68 }
                 . apply_pin (_self) . accessible_role (index - 50u32 + 1) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (32u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
                 + sp :: r#SimpleText :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get ()) , (33u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (33u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (33u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (33u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text }
                ) . apply_pin (_self) . get ()) , (34u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (34u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (34u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (34u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text }
                ) . apply_pin (_self) . get ()) , (35u32 , sp :: AccessibleStringProperty :: r#Checkable) => sp :: Some (if false {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (35u32 , sp :: AccessibleStringProperty :: r#Checked) => sp :: Some (if ({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_checked }
                ) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (35u32 , sp :: AccessibleStringProperty :: r#Enabled) => sp :: Some (if ({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                     (sp :: SharedString :: from ("true")) as _ }
                 else {
                     sp :: SharedString :: from ("false") }
                ) , (35u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: Some (({
                     InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                 + {
                     * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_text }
                ) . apply_pin (_self) . get ()) , (33u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_66 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (36u32 ..= 42u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_66 }
                 . apply_pin (_self) . accessible_string_property (index - 36u32 + 1 , what) , (34u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_67 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (43u32 ..= 49u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_67 }
                 . apply_pin (_self) . accessible_string_property (index - 43u32 + 1 , what) , (35u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_68 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (50u32 ..= 56u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_68 }
                 . apply_pin (_self) . accessible_string_property (index - 50u32 + 1 , what) , _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 (33u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
                     + {
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (34u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
                     + {
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (35u32 , sp :: AccessibilityAction :: r#Default) => {
                     ({
                         InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
                     + {
                         * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7_accessible_action_default }
                    ) . apply_pin (_self) . call (& ()) }
                 (33u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_66 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (36u32 ..= 42u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_66 }
                 . apply_pin (_self) . accessibility_action (index - 36u32 + 1 , action) , (34u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_67 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (43u32 ..= 49u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_67 }
                 . apply_pin (_self) . accessibility_action (index - 43u32 + 1 , action) , (35u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_68 }
                 . apply_pin (_self) . accessibility_action (0 , action) , (50u32 ..= 56u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#button_68 }
                 . apply_pin (_self) . accessibility_action (index - 50u32 + 1 , action) , _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 33u32 => sp :: SupportedAccessibilityAction :: r#Default , 34u32 => sp :: SupportedAccessibilityAction :: r#Default , 35u32 => sp :: SupportedAccessibilityAction :: r#Default , 33u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_66 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 36u32 ..= 42u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_66 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 36u32 + 1) , 34u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_67 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 43u32 ..= 49u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_67 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 43u32 + 1) , 35u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_68 }
                 . apply_pin (_self) . supported_accessibility_actions (0) , 50u32 ..= 56u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_68 }
                 . apply_pin (_self) . supported_accessibility_actions (index - 50u32 + 1) , _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 36u32 ..= 42u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_66 }
                 . apply_pin (_self) . item_element_infos (index - 36u32 + 1) , 43u32 ..= 49u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_67 }
                 . apply_pin (_self) . item_element_infos (index - 43u32 + 1) , 50u32 ..= 56u32 => {
                     * & Self :: FIELD_OFFSETS . r#button_68 }
                 . apply_pin (_self) . item_element_infos (index - 50u32 + 1) , _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         # [allow (dead_code , unused)] pub fn r#fn_tile_to_img (self : :: core :: pin :: Pin < & Self > , arg_0 : r#Tile ,) -> sp :: Image {
             let _self = self ;
             let args = (arg_0 ,) ;
             (if (args . 0 . clone ()) . r#flagged {
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_6 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
             else {
                 if (args . 0 . clone ()) . r#visible {
                     (if sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#value as f64) , & (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_mine_value }
                    ) . apply_pin (_self) . get () as f64)) {
                         (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_7 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                     else {
                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#value as f64) , & (0f64 as f64)) {
                             (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_8 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                         else {
                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#value as f64) , & (1f64 as f64)) {
                                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_9 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                             else {
                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#value as f64) , & (2f64 as f64)) {
                                     (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_10 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                                 else {
                                     if sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#value as f64) , & (3f64 as f64)) {
                                         (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_11 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                                     else {
                                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#value as f64) , & (4f64 as f64)) {
                                             (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_12 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                                         else {
                                             if sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#value as f64) , & (5f64 as f64)) {
                                                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_13 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                                             else {
                                                 if sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#value as f64) , & (6f64 as f64)) {
                                                     (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_14 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                                                 else {
                                                     if sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#value as f64) , & (7f64 as f64)) {
                                                         (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_15 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                                                     else {
                                                         if sp :: ApproxEq :: < f64 > :: approx_eq (& ((args . 0 . clone ()) . r#value as f64) , & (8f64 as f64)) {
                                                             (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_16 . into () , sp :: Slice :: from_slice (b"svg"))) as _ }
                                                         else {
                                                             sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_17 . into () , sp :: Slice :: from_slice (b"svg")) }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                             }
                         }
                    ) as _ }
                 else {
                     sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_17 . into () , sp :: Slice :: from_slice (b"svg")) }
                 }
            ) as _ }
         # [allow (dead_code , unused)] pub fn r#fn_touch_area_42_update_saved_values (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_42_pressed_value }
                ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_y) . apply_pin (_self) . get () . get ()) as sp :: Coord) as _) ;
                 ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_42_saved_maximum }
                ) . apply_pin (_self) . set (sp :: LogicalLength :: new (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_vertical_bar_39_maximum }
                ) . apply_pin (_self) . get () . get () as sp :: Coord) as _) ;
                 ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_42_saved_y }
                ) . apply_pin (_self) . set (sp :: LogicalLength :: new (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_42 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as sp :: Coord) as _) ;
                 ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_42_saved_x }
                ) . apply_pin (_self) . set (sp :: LogicalLength :: new (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_42 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as sp :: Coord) as _) }
            ) ;
             }
         # [allow (dead_code , unused)] pub fn r#fn_touch_area_55_update_saved_values (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             ({
                 ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_55_pressed_value }
                ) . apply_pin (_self) . set (sp :: LogicalLength :: new ((- ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_x) . apply_pin (_self) . get () . get ()) as sp :: Coord) as _) ;
                 ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_55_saved_maximum }
                ) . apply_pin (_self) . set (sp :: LogicalLength :: new (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_horizontal_bar_52_maximum }
                ) . apply_pin (_self) . get () . get () as sp :: Coord) as _) ;
                 ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_55_saved_y }
                ) . apply_pin (_self) . set (sp :: LogicalLength :: new (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_55 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_y) . apply_pin (_self) . get () . get () as sp :: Coord) as _) ;
                 ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_touch_area_55_saved_x }
                ) . apply_pin (_self) . set (sp :: LogicalLength :: new (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_55 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_x) . apply_pin (_self) . get () . get () as sp :: Coord) as _) }
            ) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_empty_32 {
         r#empty_32 : sp :: r#Empty , r#model_data : sp :: Property < sp :: ModelRc < r#Tile > > , r#model_index : sp :: Property < i32 > , r#empty_32_height : sp :: Property < sp :: LogicalLength > , r#empty_32_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#empty_32_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#empty_32_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#empty_32_y : sp :: Property < sp :: LogicalLength > , repeater0 : sp :: Repeater < InnerComponent_rectangle_33 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_empty_32 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_empty_32 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) as _ }
                 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#empty_32_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_31_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [1usize] as usize) + ({
                             * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#empty_32_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 1usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerComponent_empty_32 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_33 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 0f64 as _ ;
                                 the_struct . r#end = 0f64 as _ ;
                                 the_struct }
                             as _ , r#size : (_self . parent . upgrade () . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_31_width) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () . get () as _ , r#spacing : 0f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#empty_32_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerComponent_empty_32 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_33 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 0f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#empty_32_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len ()) ;
                         InnerComponent_empty_32 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_rectangle_33 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#empty_32_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_26_empty_31_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [0usize] as usize) + ({
                             * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerComponent_empty_32 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_33 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#empty_32_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#empty_32_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerComponent_empty_32 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_33 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerComponent_empty_32 :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_33 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             ({
                 * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#empty_32_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , _self . parent . upgrade () . as_ref () . map (| x | ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
                 + sp :: r#Flickable :: FIELD_OFFSETS . r#viewport_width) . apply_pin (x . as_pin_ref ())) . map (| x | x . get ()) . unwrap_or_default () . get () as sp :: Coord , 0f64 as sp :: Coord , ({
                     * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#empty_32_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_33 {
         r#rectangle_33 : sp :: r#Empty , r#btn_img_34 : sp :: r#ImageItem , r#touch_35 : sp :: r#TouchArea , r#model_data : sp :: Property < r#Tile > , r#model_index : sp :: Property < i32 > , r#rectangle_33_btn_img_34_preferred_height : sp :: Property < sp :: LogicalLength > , r#rectangle_33_btn_img_34_preferred_width : sp :: Property < sp :: LogicalLength > , r#rectangle_33_x : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_rectangle_33 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_empty_32 > , globals : sp :: OnceCell < sp :: Rc < SharedGlobals >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_33 {
         fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , globals : sp :: Rc < SharedGlobals > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             let _ = _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             let _ = _self . globals . set (globals) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#rectangle_33_btn_img_34_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#btn_img_34 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#rectangle_33_btn_img_34_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#btn_img_34 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , & _self . globals . get () . unwrap () . window_adapter_impl () , & & sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 1u32 - 1))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#rectangle_33_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((_self . parent . upgrade () . as_ref () . map (| x | (InnerComponent_empty_32 :: FIELD_OFFSETS . r#empty_32_layout_cache) . apply_pin (x . as_pin_ref ()))) . map (| x | {
                         let cache = x . get () ;
                         * cache . get ((cache [0usize] as usize) + ({
                             * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_index }
                        ) . apply_pin (_self) . get () as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                    ) . unwrap_or_default () as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#btn_img_34 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#btn_img_34 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (_self . parent . upgrade () . and_then (| x | x . parent . upgrade ()) . as_ref () . map (| x | x . as_pin_ref () . r#fn_tile_to_img (({
                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get () as _)) . unwrap_or_default ()) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#btn_img_34 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (40f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#touch_35 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((({
                         let r#tmp_root_26_state = (_self . parent . upgrade () . and_then (| x | x . parent . upgrade ()) . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_26_state) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default () ;
                         ((((r#tmp_root_26_state . clone ())) == ((r#GameState :: r#Initial)))) || ((((r#tmp_root_26_state . clone ())) == ((r#GameState :: r#Normal)))) }
                    )) && ((! ((((({
                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) . r#visible)) && ((sp :: ApproxEq :: < f64 > :: approx_eq (& ((({
                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                    ) . apply_pin (_self) . get ()) . r#value as f64) , & (0f64 as f64))))))))) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#touch_35 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#pointer_event) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if (((((args . 0 . clone ()) . r#button)) == ((sp :: r#PointerEventButton :: r#Right)))) && (((((args . 0 . clone ()) . r#kind)) == ((sp :: r#PointerEventKind :: r#Up)))) {
                                 ({
                                     if ((! (({
                                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                    ) . apply_pin (_self) . get ()) . r#visible)) && ((! (({
                                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                    ) . apply_pin (_self) . get ()) . r#flagged)) {
                                         ({
                                             {
                                                 * & InnerComponent_empty_32 :: FIELD_OFFSETS . repeater0 }
                                             . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . model_set_row_data (({
                                                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_index }
                                            ) . apply_pin (_self) . get () as _ , {
                                                 let r#struct_assignment0 = ({
                                                     * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                                ) . apply_pin (_self) . get () ;
                                                 {
                                                     let mut the_struct = r#Tile :: default () ;
                                                     the_struct . r#flagged = true as _ ;
                                                     the_struct . r#position = (r#struct_assignment0 . clone ()) . r#position as _ ;
                                                     the_struct . r#value = (r#struct_assignment0 . clone ()) . r#value as _ ;
                                                     the_struct . r#visible = (r#struct_assignment0 . clone ()) . r#visible as _ ;
                                                     the_struct }
                                                 }
                                             as _) ;
                                             {
                                                 let _ = (_self . parent . upgrade () . and_then (| x | x . parent . upgrade ()) . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_26_change_flag) . apply_pin (x . as_pin_ref ()))) . map (| x | x . call (& ((({
                                                     * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                                ) . apply_pin (_self) . get ()) . r#position as _ , (({
                                                     * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                                ) . apply_pin (_self) . get ()) . r#flagged as _ ,))) ;
                                                 }
                                             }
                                        ) ;
                                         }
                                     else {
                                         if ((! (({
                                             * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                        ) . apply_pin (_self) . get ()) . r#visible)) && (((({
                                             * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                        ) . apply_pin (_self) . get ()) . r#flagged)) {
                                             ({
                                                 {
                                                     * & InnerComponent_empty_32 :: FIELD_OFFSETS . repeater0 }
                                                 . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . model_set_row_data (({
                                                     * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_index }
                                                ) . apply_pin (_self) . get () as _ , {
                                                     let r#struct_assignment1 = ({
                                                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                                    ) . apply_pin (_self) . get () ;
                                                     {
                                                         let mut the_struct = r#Tile :: default () ;
                                                         the_struct . r#flagged = false as _ ;
                                                         the_struct . r#position = (r#struct_assignment1 . clone ()) . r#position as _ ;
                                                         the_struct . r#value = (r#struct_assignment1 . clone ()) . r#value as _ ;
                                                         the_struct . r#visible = (r#struct_assignment1 . clone ()) . r#visible as _ ;
                                                         the_struct }
                                                     }
                                                 as _) ;
                                                 {
                                                     let _ = (_self . parent . upgrade () . and_then (| x | x . parent . upgrade ()) . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_26_change_flag) . apply_pin (x . as_pin_ref ()))) . map (| x | x . call (& ((({
                                                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                                    ) . apply_pin (_self) . get ()) . r#position as _ , (({
                                                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                                    ) . apply_pin (_self) . get ()) . r#flagged as _ ,))) ;
                                                     }
                                                 }
                                            ) ;
                                             }
                                         else {
                                             {
                                                 }
                                             }
                                         }
                                     ;
                                     {
                                         let _ = (_self . parent . upgrade () . and_then (| x | x . parent . upgrade ()) . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_26_check_win) . apply_pin (x . as_pin_ref ()))) . map (| x | x . call (& ())) ;
                                         }
                                     }
                                ) ;
                                 }
                             else {
                                 if (((((args . 0 . clone ()) . r#button)) == ((sp :: r#PointerEventButton :: r#Left)))) && (((((args . 0 . clone ()) . r#kind)) == ((sp :: r#PointerEventKind :: r#Up)))) {
                                     ({
                                         if (({
                                             * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                        ) . apply_pin (_self) . get ()) . r#flagged {
                                             ({
                                                 {
                                                     * & InnerComponent_empty_32 :: FIELD_OFFSETS . repeater0 }
                                                 . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . model_set_row_data (({
                                                     * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_index }
                                                ) . apply_pin (_self) . get () as _ , {
                                                     let r#struct_assignment2 = ({
                                                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                                    ) . apply_pin (_self) . get () ;
                                                     {
                                                         let mut the_struct = r#Tile :: default () ;
                                                         the_struct . r#flagged = false as _ ;
                                                         the_struct . r#position = (r#struct_assignment2 . clone ()) . r#position as _ ;
                                                         the_struct . r#value = (r#struct_assignment2 . clone ()) . r#value as _ ;
                                                         the_struct . r#visible = (r#struct_assignment2 . clone ()) . r#visible as _ ;
                                                         the_struct }
                                                     }
                                                 as _) ;
                                                 {
                                                     let _ = (_self . parent . upgrade () . and_then (| x | x . parent . upgrade ()) . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_26_change_flag) . apply_pin (x . as_pin_ref ()))) . map (| x | x . call (& ((({
                                                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                                    ) . apply_pin (_self) . get ()) . r#position as _ , (({
                                                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                                    ) . apply_pin (_self) . get ()) . r#flagged as _ ,))) ;
                                                     }
                                                 }
                                            ) ;
                                             }
                                         else {
                                             {
                                                 if (((_self . parent . upgrade () . and_then (| x | x . parent . upgrade ()) . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_26_state) . apply_pin (x . as_pin_ref ()))) . map (| x | x . get ()) . unwrap_or_default ())) == ((r#GameState :: r#Initial)) {
                                                     ({
                                                         {
                                                             let _ = (_self . parent . upgrade () . and_then (| x | x . parent . upgrade ()) . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_26_first_move_occured) . apply_pin (x . as_pin_ref ()))) . map (| x | x . call (& ((({
                                                                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                                            ) . apply_pin (_self) . get ()) . r#position as _ ,))) ;
                                                             }
                                                         }
                                                    ) ;
                                                     }
                                                 else {
                                                     {
                                                         }
                                                     }
                                                 ;
                                                 {
                                                     * & InnerComponent_empty_32 :: FIELD_OFFSETS . repeater0 }
                                                 . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . model_set_row_data (({
                                                     * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_index }
                                                ) . apply_pin (_self) . get () as _ , {
                                                     let r#struct_assignment3 = ({
                                                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                                    ) . apply_pin (_self) . get () ;
                                                     {
                                                         let mut the_struct = r#Tile :: default () ;
                                                         the_struct . r#flagged = (r#struct_assignment3 . clone ()) . r#flagged as _ ;
                                                         the_struct . r#position = (r#struct_assignment3 . clone ()) . r#position as _ ;
                                                         the_struct . r#value = (r#struct_assignment3 . clone ()) . r#value as _ ;
                                                         the_struct . r#visible = true as _ ;
                                                         the_struct }
                                                     }
                                                 as _) ;
                                                 {
                                                     let _ = (_self . parent . upgrade () . and_then (| x | x . parent . upgrade ()) . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_26_change_visibility) . apply_pin (x . as_pin_ref ()))) . map (| x | x . call (& ((({
                                                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                                    ) . apply_pin (_self) . get ()) . r#position as _ , true as _ ,))) ;
                                                     }
                                                 ;
                                                 {
                                                     let _ = (_self . parent . upgrade () . and_then (| x | x . parent . upgrade ()) . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_26_expand_selection) . apply_pin (x . as_pin_ref ()))) . map (| x | x . call (& ((({
                                                         * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
                                                    ) . apply_pin (_self) . get ()) . r#position as _ ,))) ;
                                                     }
                                                 }
                                             }
                                         ;
                                         {
                                             let _ = (_self . parent . upgrade () . and_then (| x | x . parent . upgrade ()) . as_ref () . map (| x | (InnerMainWindow :: FIELD_OFFSETS . r#root_26_check_win) . apply_pin (x . as_pin_ref ()))) . map (| x | x . call (& ())) ;
                                             }
                                         }
                                    ) ;
                                     }
                                 else {
                                     {
                                         }
                                     }
                                 }
                             }
                        ) ;
                         }
                     }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#btn_img_34 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#btn_img_34 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#btn_img_34 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#btn_img_34 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#btn_img_34 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#touch_35 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             }
         fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#rectangle_33_btn_img_34_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                    )) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 40f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 40f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => {
                     let r#layout_info = (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    )) + (({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#rectangle_33_btn_img_34_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = 0f64 as _ ;
                         the_struct }
                    )) ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 40f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 40f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . get () as usize }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (40f64 as sp :: Coord , 40f64 as sp :: Coord , ({
                     * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#rectangle_33_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord ,) , 1u32 => (40f64 as sp :: Coord , 40f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , 2u32 => (40f64 as sp :: Coord , 40f64 as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Image , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => sp :: None , }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             # ! [allow (unused)] let _self = self ;
             match (index , action) {
                 _ => () , }
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => :: core :: default :: Default :: default () , }
             }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: Option < sp :: SharedString > {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => {
                     :: core :: default :: Default :: default () }
                 }
             }
         }
     impl InnerComponent_rectangle_33 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_empty_32 > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerComponent_empty_32 > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             3usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 2u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_rectangle_33 , sp :: ItemVTable , sp :: AllowPin > ;
             3usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#rectangle_33 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#btn_img_34 }
            ) , sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#touch_35 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_rectangle_33) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_rectangle_33 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_rectangle_33 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_rectangle_33 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_33 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 1u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_rectangle_33 {
         type Data = r#Tile ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . set (_index as _) ;
             ({
                 * & InnerComponent_rectangle_33 :: FIELD_OFFSETS . r#model_data }
            ) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_33 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerComponent_empty_32 {
         fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > ,) -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = parent . upgrade () . unwrap () . globals . get () . unwrap () . clone () ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             2usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_empty_32 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#empty_32 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_empty_32) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_empty_32 {
         fn drop (self : :: core :: pin :: Pin < & mut InnerComponent_empty_32 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_empty_32 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_empty_32 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 7u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     impl sp :: RepeatedItemTree for InnerComponent_empty_32 {
         type Data = sp :: ModelRc < r#Tile > ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             ({
                 * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#model_index }
            ) . apply_pin (_self) . set (_index as _) ;
             ({
                 * & InnerComponent_empty_32 :: FIELD_OFFSETS . r#model_data }
            ) . apply_pin (_self) . set (_data) ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_empty_32 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerMainWindow {
         fn new () -> :: core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             let globals = SharedGlobals :: new (sp :: VRc :: downgrade (& self_dyn_rc)) ;
             sp :: register_item_tree (& self_dyn_rc , globals . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , globals , 0 , 1) ;
             :: core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             57usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 32u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 6u32 , parent_index : 1u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 8u32 , parent_index : 1u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 20u32 , parent_index : 1u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 7u32 , parent_index : 3u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 9u32 , parent_index : 4u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 10u32 , parent_index : 8u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 9u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 14u32 , parent_index : 9u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 14u32 , parent_index : 9u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 17u32 , parent_index : 9u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 15u32 , parent_index : 12u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 16u32 , parent_index : 14u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 17u32 , parent_index : 15u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 18u32 , parent_index : 13u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 19u32 , parent_index : 17u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 20u32 , parent_index : 18u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 21u32 , parent_index : 5u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 4u32 , children_index : 22u32 , parent_index : 20u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 26u32 , parent_index : 21u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 26u32 , parent_index : 21u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 26u32 , parent_index : 21u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 29u32 , parent_index : 21u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 27u32 , parent_index : 24u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 28u32 , parent_index : 26u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 29u32 , parent_index : 27u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 30u32 , parent_index : 25u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 31u32 , parent_index : 29u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 32u32 , parent_index : 30u32 , item_array_index : 30u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 36u32 , parent_index : 2u32 , item_array_index : 31u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 36u32 , parent_index : 2u32 , item_array_index : 32u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 43u32 , parent_index : 2u32 , item_array_index : 33u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 4u32 , children_index : 50u32 , parent_index : 2u32 , item_array_index : 34u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 40u32 , parent_index : 33u32 , item_array_index : 35u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 43u32 , parent_index : 33u32 , item_array_index : 36u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 43u32 , parent_index : 33u32 , item_array_index : 37u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 3u32 , parent_index : 33u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 43u32 , parent_index : 36u32 , item_array_index : 38u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 36u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 36u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 47u32 , parent_index : 34u32 , item_array_index : 39u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 50u32 , parent_index : 34u32 , item_array_index : 40u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 50u32 , parent_index : 34u32 , item_array_index : 41u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 6u32 , parent_index : 34u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 50u32 , parent_index : 43u32 , item_array_index : 42u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 4u32 , parent_index : 43u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 5u32 , parent_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 54u32 , parent_index : 35u32 , item_array_index : 43u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 57u32 , parent_index : 35u32 , item_array_index : 44u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 57u32 , parent_index : 35u32 , item_array_index : 45u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 9u32 , parent_index : 35u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 57u32 , parent_index : 50u32 , item_array_index : 46u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 7u32 , parent_index : 50u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 8u32 , parent_index : 50u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerMainWindow , sp :: ItemVTable , sp :: AllowPin > ;
             47usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: vec ! [sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#scroll_28 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#empty_64 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_29 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_visibility_38 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_visibility_51 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#flickable_viewport_30 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_39 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#vertical_bar_clip_40 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#thumb_41 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_42 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_Opacity_43 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_Opacity_47 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_44 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_Opacity_45 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_46 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_48 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_Opacity_49 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_50 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_52 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#horizontal_bar_clip_53 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#thumb_54 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#touch_area_55 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_Opacity_56 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_Opacity_60 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#up_scroll_button_57 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_Opacity_58 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_59 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#down_scroll_button_61 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_Opacity_62 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#icon_63 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_65 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#root_7 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_background_8 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_touch_area_15 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_66 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_border_9 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_background_8 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_touch_area_15 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_67 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_border_9 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_background_8 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_touch_area_15 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_focus_scope_16 }
            ) , sp :: VOffset :: new ({
                 InnerMainWindow :: FIELD_OFFSETS . r#button_68 }
             + {
                 * & InnerButton_root_7 :: FIELD_OFFSETS . r#i_border_9 }
            )] . into_boxed_slice () . try_into () . unwrap ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerMainWindow) ;
         }
     ;
     impl sp :: PinnedDrop for InnerMainWindow {
         fn drop (self : :: core :: pin :: Pin < & mut InnerMainWindow >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . globals . get () . unwrap () . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerMainWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerMainWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) -> bool {
             if let Some (r) = self . accessible_string_property (index , what) {
                 * result = r ;
                 true }
             else {
                 false }
             }
         fn accessibility_action (self : :: core :: pin :: Pin < & Self > , index : u32 , action : & sp :: AccessibilityAction) {
             self . accessibility_action (index , action) ;
             }
         fn supported_accessibility_actions (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: SupportedAccessibilityAction {
             self . supported_accessibility_actions (index) }
         fn item_element_infos (self : :: core :: pin :: Pin < & Self > , _index : u32 , _result : & mut sp :: SharedString ,) -> bool {
             false }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . globals . get () . unwrap () . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . globals . get () . unwrap () . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#MainWindow (sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow >) ;
     impl r#MainWindow {
         pub fn new () -> :: core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerMainWindow :: new () ? ;
             inner . globals . get () . unwrap () . window_adapter_ref () ? ;
             InnerMainWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             :: core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] pub fn invoke_about (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_about }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_about (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_about }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_change_flag (& self , arg_0 : r#Position , arg_1 : bool ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_change_flag }
            ) . apply_pin (_self) . call (& (arg_0 , arg_1 ,)) }
         # [allow (dead_code)] pub fn on_change_flag (& self , mut f : impl FnMut (r#Position , bool) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_change_flag }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone () , args . 1 . clone ())) }
         # [allow (dead_code)] pub fn invoke_change_visibility (& self , arg_0 : r#Position , arg_1 : bool ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_change_visibility }
            ) . apply_pin (_self) . call (& (arg_0 , arg_1 ,)) }
         # [allow (dead_code)] pub fn on_change_visibility (& self , mut f : impl FnMut (r#Position , bool) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_change_visibility }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone () , args . 1 . clone ())) }
         # [allow (dead_code)] pub fn invoke_check_win (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_check_win }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_check_win (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_check_win }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_close (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_close }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_close (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_close }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn invoke_expand_selection (& self , arg_0 : r#Position ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_expand_selection }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_expand_selection (& self , mut f : impl FnMut (r#Position) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_expand_selection }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn invoke_first_move_occured (& self , arg_0 : r#Position ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_first_move_occured }
            ) . apply_pin (_self) . call (& (arg_0 ,)) }
         # [allow (dead_code)] pub fn on_first_move_occured (& self , mut f : impl FnMut (r#Position) -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_first_move_occured }
            ) . apply_pin (_self) . set_handler (move | args | f (args . 0 . clone ())) }
         # [allow (dead_code)] pub fn get_grid (& self) -> sp :: ModelRc < sp :: ModelRc < r#Tile > > {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_grid }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_grid (& self , value : sp :: ModelRc < sp :: ModelRc < r#Tile > >) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_grid }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn get_mine_value (& self) -> i32 {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_mine_value }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_mine_value (& self , value : i32) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_mine_value }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_restart (& self ,) -> () {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_restart }
            ) . apply_pin (_self) . call (& ()) }
         # [allow (dead_code)] pub fn on_restart (& self , mut f : impl FnMut () -> () + 'static) {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             # [allow (unused)] ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_restart }
            ) . apply_pin (_self) . set_handler (move | args | f ()) }
         # [allow (dead_code)] pub fn get_state (& self) -> r#GameState {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_state }
            ) . apply_pin (_self) . get () }
         # [allow (dead_code)] pub fn set_state (& self , value : r#GameState) {
             # [allow (unused_imports)] let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_26_state }
            ) . apply_pin (_self) . set (value as _) }
         # [allow (dead_code)] pub fn invoke_tile_to_img (& self , arg_0 : r#Tile ,) -> sp :: Image {
             let _self = sp :: VRc :: as_pin_ref (& self . 0) ;
             _self . r#fn_tile_to_img (arg_0 ,) }
         }
     impl From < r#MainWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow > {
         fn from (value : r#MainWindow) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#MainWindow {
         type WeakInner = sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow > ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (sp :: VRc :: downgrade (& self . 0)) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn upgrade_from_weak_inner (inner : & Self :: WeakInner) -> sp :: Option < Self > {
             sp :: Some (Self (inner . upgrade () ?)) }
         fn run (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             :: core :: result :: Result :: Ok (()) }
         fn show (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> :: core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . globals . get () . unwrap () . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     struct SharedGlobals {
         global_FluentPalette_70 : :: core :: pin :: Pin < sp :: Rc < InnerFluentPalette_70 >> , window_adapter : sp :: OnceCell < sp :: WindowAdapterRc > , root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable > , }
     impl SharedGlobals {
         fn new (root_item_tree_weak : sp :: VWeak < sp :: ItemTreeVTable >) -> sp :: Rc < Self > {
             let _self = sp :: Rc :: new (Self {
                 global_FluentPalette_70 : InnerFluentPalette_70 :: new () , window_adapter : :: core :: default :: Default :: default () , root_item_tree_weak , }
            ) ;
             _self . global_FluentPalette_70 . clone () . init (& _self) ;
             _self }
         fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let root_rc = self . root_item_tree_weak . upgrade () . unwrap () ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& root_rc) ;
                 :: core :: result :: Result :: Ok (adapter) }
            ) }
         fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter . get () . cloned () }
         }
     static SLINT_EMBEDDED_RESOURCE_9 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/1.svg") ;
     static SLINT_EMBEDDED_RESOURCE_10 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/2.svg") ;
     static SLINT_EMBEDDED_RESOURCE_11 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/3.svg") ;
     static SLINT_EMBEDDED_RESOURCE_12 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/4.svg") ;
     static SLINT_EMBEDDED_RESOURCE_13 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/5.svg") ;
     static SLINT_EMBEDDED_RESOURCE_14 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/6.svg") ;
     static SLINT_EMBEDDED_RESOURCE_15 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/7.svg") ;
     static SLINT_EMBEDDED_RESOURCE_16 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/8.svg") ;
     static SLINT_EMBEDDED_RESOURCE_8 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/blank.svg") ;
     static SLINT_EMBEDDED_RESOURCE_17 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/empty.svg") ;
     static SLINT_EMBEDDED_RESOURCE_21 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/exit.svg") ;
     static SLINT_EMBEDDED_RESOURCE_6 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/flag.svg") ;
     static SLINT_EMBEDDED_RESOURCE_2 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/info.svg") ;
     static SLINT_EMBEDDED_RESOURCE_5 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/minesweeper.svg") ;
     static SLINT_EMBEDDED_RESOURCE_20 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/reset.svg") ;
     static SLINT_EMBEDDED_RESOURCE_7 : & 'static [u8] = :: core :: include_bytes ! ("/home/ben/Projects/Rust/minesweeper-slint/src/mine_sweeper_ui/resources/icons/warning.svg") ;
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = b"<svg width=\"212\" height=\"97\" viewBox=\"0 0 212 97\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect width=\"212\" height=\"97\" rx=\"48.5\" fill=\"white\"/>\n<path d=\"M44.116 54.8572L65.6681 39.826C65.6681 39.826 66.6407 39.2638 66.6407 38.3765C66.6407 37.1949 65.4108 36.8106 65.4108 36.8106L53.5542 32.0886C53.1311 31.9217 52.5489 32.3895 53.094 32.9825L57.0193 36.9358C57.0193 36.9358 58.1097 38.0141 58.1097 38.7213C58.1097 39.4285 57.4402 40.0588 57.4402 40.0588L43.1565 53.948C42.6484 54.4421 43.3353 55.3382 44.116 54.8572Z\" fill=\"#2379F4\"/>\n<path d=\"M61.5247 16.75L39.9726 31.779C39.9726 31.779 39 32.3412 39 33.2285C39 34.4101 40.2299 34.7944 40.2299 34.7944L52.0865 39.5186C52.5096 39.6833 53.094 39.2155 52.5467 38.6247L48.6214 34.6583C48.6214 34.6583 47.531 33.5821 47.531 32.8727C47.531 32.1633 48.2005 31.5352 48.2005 31.5352L62.4755 17.657C62.9924 17.1629 62.3076 16.2668 61.5247 16.75Z\" fill=\"#2379F4\"/>\n<path d=\"M95.8159 40.165C94.4289 39.1164 92.4854 38.4266 89.9854 38.0955L85.3776 37.5032C84.1413 37.3497 83.1926 37.0686 82.5317 36.6599C82.2184 36.4812 81.96 36.2193 81.7847 35.9026C81.6094 35.586 81.5239 35.2269 81.5377 34.8646C81.5377 33.0125 83.2842 32.0873 86.7774 32.089C88.7633 32.0612 90.7381 32.3924 92.6079 33.0668C93.1817 33.2698 93.7341 33.5297 94.2569 33.8429C94.5363 34.0081 94.8002 34.1986 95.0454 34.412C95.193 34.3457 95.3305 34.2586 95.4538 34.1533C95.6458 34.0031 95.8183 33.8293 95.9675 33.6359C96.1418 33.4085 96.2932 33.1641 96.4195 32.9064C96.5609 32.6088 96.6305 32.2818 96.6224 31.9519C96.6224 30.8293 95.7569 29.9187 94.0257 29.2203C92.2945 28.5219 89.8622 28.1769 86.7286 28.1856C85.1211 28.1527 83.5168 28.3442 81.9615 28.7547C80.8055 29.0563 79.7221 29.5904 78.7765 30.3248C78.0025 30.9337 77.3857 31.7217 76.9786 32.6219C76.6008 33.4893 76.4083 34.4271 76.4135 35.3742C76.3859 36.3727 76.5955 37.3633 77.0248 38.2637C77.4257 39.0617 78.0051 39.7551 78.7175 40.2891C79.5047 40.8658 80.3837 41.3032 81.3168 41.5825C82.3984 41.9165 83.5113 42.1375 84.6378 42.2421L88.1618 42.6534C89.7885 42.8259 90.9178 43.1363 91.5497 43.5847C91.8591 43.7983 92.1087 44.0885 92.2747 44.4274C92.4407 44.7662 92.5174 45.1423 92.4974 45.5196C92.4974 46.5215 92.0009 47.2269 91.0077 47.6356C90.0146 48.0443 88.7303 48.2487 87.155 48.2487C86.0363 48.2553 84.9191 48.1644 83.8159 47.977C82.8916 47.8179 81.9786 47.5976 81.0831 47.3174C80.4 47.1023 79.7287 46.8511 79.0719 46.5647C78.6781 46.4034 78.2997 46.2059 77.9418 45.9749L76 49.4774C76.3686 49.7423 76.7617 49.971 77.1738 50.1603C77.9419 50.5279 78.735 50.84 79.5471 51.0941C80.6793 51.4497 81.8332 51.7305 83.0017 51.9348C84.4639 52.1895 85.9459 52.3116 87.4298 52.2996C90.8014 52.2996 93.3844 51.6166 95.1789 50.2508C96.0362 49.6313 96.7296 48.8096 97.1985 47.8576C97.6673 46.9056 97.8972 45.8524 97.8682 44.7901C97.8853 42.7552 97.2012 41.2135 95.8159 40.165Z\" fill=\"#151D21\"/>\n<path d=\"M106.298 18.1178C105.425 18.1178 104.757 18.317 104.264 18.7102C103.771 19.1034 103.542 19.7113 103.542 20.5209V51.6554H108.692V18.6196C108.54 18.5601 108.24 18.4593 107.788 18.3247C107.304 18.1825 106.802 18.1128 106.298 18.1178Z\" fill=\"#151D21\"/>\n<path d=\"M118.198 18.5731C117.787 18.5682 117.379 18.6461 116.999 18.8022C116.618 18.9583 116.273 19.1895 115.982 19.4821C115.692 19.7746 115.462 20.1228 115.307 20.506C115.152 20.8892 115.075 21.2997 115.08 21.7134C115.082 22.2288 115.211 22.7356 115.454 23.1891C115.697 23.6426 116.047 24.0289 116.473 24.3139C116.9 24.5989 117.389 24.7739 117.899 24.8233C118.408 24.8728 118.921 24.7953 119.394 24.5975C119.867 24.3998 120.284 24.088 120.608 23.6897C120.933 23.2913 121.155 22.8186 121.255 22.3132C121.356 21.8078 121.331 21.2854 121.183 20.7919C121.036 20.2985 120.77 19.8491 120.409 19.4836C120.122 19.1879 119.777 18.9545 119.397 18.7979C119.017 18.6413 118.609 18.5648 118.198 18.5731V18.5731Z\" fill=\"#151D21\"/>\n<path d=\"M118.37 28.8168C117.497 28.8168 116.829 29.0082 116.336 29.3858C115.843 29.7635 115.622 30.3688 115.622 31.2225V51.658H120.771V29.3134C120.62 29.2513 120.319 29.153 119.867 29.0159C119.38 28.877 118.876 28.8099 118.37 28.8168Z\" fill=\"#151D21\"/>\n<path d=\"M146.384 30.7465C145.354 29.8828 144.164 29.2331 142.884 28.8348C141.451 28.3836 139.957 28.161 138.456 28.1752C136.962 28.1595 135.475 28.3822 134.051 28.8348C132.777 29.2366 131.595 29.8861 130.57 30.7465C129.588 31.5801 128.799 32.6202 128.259 33.7937C127.687 35.0744 127.405 36.4667 127.432 37.8705V51.658H132.581V39.1897C132.581 37.1203 133.071 35.5501 134.051 34.4792C135.03 33.4083 136.498 32.8702 138.456 32.865C140.382 32.865 141.842 33.4031 142.837 34.4792C143.832 35.5553 144.33 37.1255 144.33 39.1897V51.658H149.479V37.8705C149.507 36.4693 149.234 35.0786 148.678 33.7937C148.15 32.6177 147.366 31.5763 146.384 30.7465V30.7465Z\" fill=\"#151D21\"/>\n<path d=\"M168.409 46.5129C167.903 46.8416 167.357 47.1025 166.783 47.289C166.001 47.5667 165.176 47.7058 164.346 47.7003C163.821 47.7026 163.298 47.6418 162.787 47.5192C162.309 47.4066 161.862 47.1893 161.477 46.8828C161.087 46.5611 160.784 46.1457 160.596 45.6748C160.365 45.058 160.258 44.4013 160.28 43.7425V33.5894H168.954C169.092 33.2861 169.213 32.9752 169.316 32.6581C169.469 32.1954 169.546 31.7103 169.542 31.2224C169.542 30.4464 169.339 29.8359 168.931 29.4272C168.522 29.0185 167.821 28.8167 166.827 28.8167H160.277V22.8671C160.126 22.8051 159.818 22.7068 159.35 22.5697C158.858 22.4308 158.349 22.362 157.837 22.3653C157.13 22.3252 156.43 22.5256 155.849 22.9344C155.356 23.2991 155.125 23.9019 155.125 24.7633V44.7824C155.125 47.0242 155.803 48.835 157.159 50.2146C158.515 51.5942 160.519 52.284 163.17 52.284C164.032 52.287 164.893 52.2004 165.738 52.0254C166.46 51.8751 167.17 51.6701 167.862 51.4123C168.419 51.2043 168.956 50.9445 169.465 50.6363C169.902 50.3621 170.244 50.1344 170.492 49.9533L168.409 46.5129Z\" fill=\"#151D21\"/>\n<path d=\"M40.472 78L41.6458 73.9195H39.559V72.8388H41.9439L42.7265 70.1185H40.4906V69.0379H43.0246L44.1984 64.9574H45.4654L44.2916 69.0379H47.9808L49.1546 64.9574H50.4216L49.2478 69.0379H51.3346V70.1185H48.9497L48.1671 72.8388H50.403V73.9195H47.869L46.6951 78H45.4282L46.602 73.9195H42.9128L41.739 78H40.472ZM43.2109 72.8388H46.9001L47.6827 70.1185H43.9935L43.2109 72.8388ZM59.7822 77.0684L54.6769 66.6157V78H53.3354V64.9574H55.3477L59.8008 74.2176L64.2353 64.9574H66.2476V78H64.8874V66.597L59.7822 77.0684ZM71.6121 78.1677C70.5935 78.1677 69.7613 77.9068 69.1154 77.3851C68.4819 76.8634 68.1651 76.1865 68.1651 75.3542C68.1651 74.4847 68.5005 73.7953 69.1713 73.286C69.842 72.7643 70.7488 72.5035 71.8916 72.5035C72.3884 72.5035 72.8667 72.5594 73.3263 72.6712C73.7983 72.7829 74.233 72.9382 74.6305 73.137V71.9072C74.6305 71.1619 74.4194 70.603 73.997 70.2303C73.5747 69.8453 72.9598 69.6527 72.1524 69.6527C71.6928 69.6527 71.227 69.721 70.755 69.8577C70.2954 69.9819 69.7861 70.1868 69.2272 70.4725L68.7055 69.4105C69.3762 69.0875 69.9973 68.8515 70.5687 68.7025C71.1401 68.5534 71.7115 68.4789 72.2829 68.4789C73.4629 68.4789 74.3697 68.7584 75.0032 69.3173C75.6491 69.8763 75.9721 70.6775 75.9721 71.7209V78H74.6305V77.087C74.2082 77.4472 73.7424 77.7205 73.2331 77.9068C72.7362 78.0807 72.1959 78.1677 71.6121 78.1677ZM69.488 75.3169C69.488 75.8387 69.7054 76.2672 70.1401 76.6026C70.5873 76.9255 71.1587 77.087 71.8543 77.087C72.4133 77.087 72.9226 77.0001 73.3822 76.8262C73.8418 76.6523 74.2579 76.379 74.6305 76.0063V74.2549C74.2455 73.9941 73.8293 73.8077 73.3822 73.6959C72.9474 73.5717 72.463 73.5096 71.9288 73.5096C71.1836 73.5096 70.5873 73.6773 70.1401 74.0127C69.7054 74.3356 69.488 74.7704 69.488 75.3169ZM85.5607 78V77.0125C85.1632 77.3851 84.7036 77.6708 84.1819 77.8696C83.6726 78.0559 83.1261 78.1491 82.5423 78.1491C81.8839 78.1491 81.2691 78.0248 80.6977 77.7764C80.1263 77.528 79.6294 77.1926 79.2071 76.7703C78.7848 76.3355 78.4494 75.82 78.2009 75.2238C77.9649 74.6275 77.8469 73.9941 77.8469 73.3233C77.8469 72.6525 77.9649 72.0252 78.2009 71.4414C78.4494 70.8452 78.7848 70.3359 79.2071 69.9136C79.6294 69.4788 80.1263 69.1372 80.6977 68.8888C81.2815 68.6404 81.9026 68.5161 82.5609 68.5161C83.095 68.5161 83.6167 68.6093 84.126 68.7956C84.6477 68.9695 85.1198 69.2304 85.5421 69.5782V64.9574L86.9209 64.6406V78H85.5607ZM79.2071 73.3047C79.2071 73.8139 79.294 74.2922 79.4679 74.7393C79.6543 75.1865 79.9027 75.5716 80.2132 75.8945C80.5238 76.2175 80.8902 76.4721 81.3125 76.6585C81.7349 76.8448 82.1883 76.938 82.6727 76.938C83.2565 76.938 83.7969 76.8262 84.2937 76.6026C84.803 76.3666 85.2191 76.0436 85.5421 75.6337V71.0129C85.2191 70.6154 84.803 70.3049 84.2937 70.0813C83.7844 69.8453 83.2441 69.7272 82.6727 69.7272C81.7038 69.7272 80.884 70.0688 80.2132 70.752C79.5425 71.4352 79.2071 72.2861 79.2071 73.3047ZM97.1285 77.0125C96.6068 77.3976 96.0602 77.6895 95.4888 77.8882C94.9299 78.0745 94.315 78.1677 93.6442 78.1677C92.9735 78.1677 92.34 78.0435 91.7437 77.795C91.1599 77.5466 90.6506 77.205 90.2159 76.7703C89.7936 76.3355 89.4582 75.8262 89.2097 75.2424C88.9613 74.6462 88.8371 74.0065 88.8371 73.3233C88.8371 72.6525 88.9551 72.0252 89.1911 71.4414C89.4395 70.8576 89.7687 70.3483 90.1786 69.9136C90.5885 69.4788 91.073 69.1372 91.6319 68.8888C92.2033 68.6404 92.8058 68.5161 93.4393 68.5161C94.0604 68.5161 94.638 68.6404 95.1721 68.8888C95.7186 69.1372 96.1844 69.4788 96.5695 69.9136C96.967 70.3483 97.2775 70.8576 97.5011 71.4414C97.7371 72.0252 97.8551 72.6525 97.8551 73.3233V73.7518H90.1973C90.2966 74.671 90.6755 75.435 91.3338 76.0436C91.9922 76.6523 92.7809 76.9566 93.7001 76.9566C94.1846 76.9566 94.6566 76.8821 95.1162 76.733C95.5758 76.5839 95.9609 76.3728 96.2714 76.0995L97.1285 77.0125ZM93.402 69.7272C92.5946 69.7272 91.8928 70.0005 91.2966 70.5471C90.7128 71.0936 90.3587 71.7954 90.2345 72.6525H96.4763C96.3521 71.8327 95.9981 71.1433 95.4143 70.5843C94.8429 70.0129 94.1722 69.7272 93.402 69.7272ZM103.528 78H102.074L99.261 64.9574H100.714L102.932 75.7269L106.155 64.9574H107.552L110.776 75.6896L113.012 64.9574H114.409L111.596 78H110.142L106.844 66.951L103.528 78ZM116.903 66.8765C116.654 66.8765 116.437 66.7833 116.251 66.597C116.064 66.4107 115.971 66.1933 115.971 65.9449C115.971 65.684 116.064 65.4667 116.251 65.2928C116.437 65.1064 116.654 65.0133 116.903 65.0133C117.164 65.0133 117.381 65.1064 117.555 65.2928C117.741 65.4667 117.834 65.684 117.834 65.9449C117.834 66.1933 117.741 66.4107 117.555 66.597C117.381 66.7833 117.164 66.8765 116.903 66.8765ZM117.592 68.6652V78H116.213V68.6652H117.592ZM121.099 75.9132V69.839H119.086V68.6652H121.099V66.2989L122.459 65.9449V68.6652H125.272V69.839H122.459V75.5592C122.459 76.056 122.571 76.4163 122.794 76.6398C123.018 76.851 123.384 76.9566 123.894 76.9566C124.154 76.9566 124.384 76.938 124.583 76.9007C124.794 76.8634 125.018 76.8013 125.254 76.7144V77.9255C125.018 78.0124 124.751 78.0745 124.452 78.1118C124.167 78.1491 123.887 78.1677 123.614 78.1677C122.807 78.1677 122.186 77.9752 121.751 77.5901C121.316 77.205 121.099 76.6461 121.099 75.9132ZM126.749 78V64.9574L128.127 64.6406V69.7645C128.488 69.3422 128.916 69.0254 129.413 68.8143C129.91 68.5907 130.463 68.4789 131.071 68.4789C132.127 68.4789 132.99 68.8143 133.661 69.485C134.332 70.1558 134.667 71.0253 134.667 72.0936V78H133.307V72.373C133.307 71.5532 133.071 70.9011 132.599 70.4166C132.127 69.9322 131.494 69.69 130.699 69.69C130.14 69.69 129.637 69.8142 129.189 70.0626C128.755 70.3111 128.401 70.6651 128.127 71.1247V78H126.749ZM136.006 75.7641L136.956 74.7766C137.627 75.4846 138.341 76.025 139.099 76.3976C139.869 76.7578 140.676 76.938 141.521 76.938C142.539 76.938 143.378 76.6957 144.036 76.2113C144.695 75.7269 145.024 75.112 145.024 74.3667C145.024 73.6959 144.788 73.1804 144.316 72.8202C143.856 72.46 143.092 72.1929 142.024 72.019L140.142 71.7209C138.887 71.5222 137.962 71.1557 137.366 70.6216C136.77 70.0751 136.471 69.3422 136.471 68.423C136.471 67.3423 136.894 66.4666 137.738 65.7958C138.583 65.1251 139.689 64.7897 141.055 64.7897C141.962 64.7897 142.862 64.9387 143.757 65.2369C144.651 65.535 145.465 65.9635 146.198 66.5225L145.378 67.6404C144.67 67.1063 143.949 66.7026 143.216 66.4293C142.484 66.1561 141.744 66.0194 140.999 66.0194C140.068 66.0194 139.31 66.2368 138.726 66.6715C138.155 67.0939 137.869 67.6404 137.869 68.3112C137.869 68.9198 138.074 69.3857 138.484 69.7086C138.894 70.0316 139.571 70.2676 140.515 70.4166L142.378 70.7148C143.794 70.9508 144.825 71.3545 145.471 71.9259C146.129 72.4973 146.458 73.286 146.458 74.2922C146.458 74.8636 146.334 75.3915 146.086 75.8759C145.85 76.3479 145.508 76.7578 145.061 77.1056C144.626 77.441 144.098 77.7081 143.477 77.9068C142.869 78.0932 142.198 78.1863 141.465 78.1863C140.434 78.1863 139.44 77.9814 138.484 77.5715C137.54 77.1491 136.714 76.5467 136.006 75.7641ZM149.525 64.6406V78H148.147V64.9574L149.525 64.6406ZM152.566 66.8765C152.318 66.8765 152.1 66.7833 151.914 66.597C151.728 66.4107 151.635 66.1933 151.635 65.9449C151.635 65.684 151.728 65.4667 151.914 65.2928C152.1 65.1064 152.318 65.0133 152.566 65.0133C152.827 65.0133 153.044 65.1064 153.218 65.2928C153.405 65.4667 153.498 65.684 153.498 65.9449C153.498 66.1933 153.405 66.4107 153.218 66.597C153.044 66.7833 152.827 66.8765 152.566 66.8765ZM153.256 68.6652V78H151.877V68.6652H153.256ZM155.607 78V68.6652H156.986V69.7645C157.346 69.3422 157.774 69.0254 158.271 68.8143C158.768 68.5907 159.321 68.4789 159.93 68.4789C160.985 68.4789 161.849 68.8143 162.519 69.485C163.19 70.1558 163.526 71.0253 163.526 72.0936V78H162.165V72.373C162.165 71.5532 161.929 70.9011 161.457 70.4166C160.985 69.9322 160.352 69.69 159.557 69.69C158.998 69.69 158.495 69.8142 158.048 70.0626C157.613 70.3111 157.259 70.6651 156.986 71.1247V78H155.607ZM166.624 75.9132V69.839H164.612V68.6652H166.624V66.2989L167.984 65.9449V68.6652H170.798V69.839H167.984V75.5592C167.984 76.056 168.096 76.4163 168.32 76.6398C168.543 76.851 168.91 76.9566 169.419 76.9566C169.68 76.9566 169.91 76.938 170.108 76.9007C170.32 76.8634 170.543 76.8013 170.779 76.7144V77.9255C170.543 78.0124 170.276 78.0745 169.978 78.1118C169.692 78.1491 169.413 78.1677 169.139 78.1677C168.332 78.1677 167.711 77.9752 167.276 77.5901C166.842 77.205 166.624 76.6461 166.624 75.9132Z\" fill=\"#151D21\"/>\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_1 : & 'static [u8] = b"<svg width=\"212\" height=\"97\" viewBox=\"0 0 212 97\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<rect width=\"212\" height=\"97\" rx=\"48.5\" fill=\"#151D21\"/>\n<path d=\"M44.116 54.8572L65.6681 39.8261C65.6681 39.8261 66.6407 39.2638 66.6407 38.3765C66.6407 37.1949 65.4108 36.8106 65.4108 36.8106L53.5542 32.0887C53.1311 31.9217 52.5489 32.3895 53.094 32.9825L57.0193 36.9358C57.0193 36.9358 58.1097 38.0142 58.1097 38.7213C58.1097 39.4285 57.4402 40.0589 57.4402 40.0589L43.1565 53.948C42.6484 54.4421 43.3353 55.3382 44.116 54.8572Z\" fill=\"#2379F4\"/>\n<path d=\"M61.5247 16.75L39.9726 31.779C39.9726 31.779 39 32.3412 39 33.2285C39 34.4101 40.2299 34.7944 40.2299 34.7944L52.0865 39.5186C52.5096 39.6833 53.094 39.2155 52.5467 38.6247L48.6214 34.6583C48.6214 34.6583 47.531 33.5821 47.531 32.8727C47.531 32.1633 48.2005 31.5352 48.2005 31.5352L62.4755 17.657C62.9924 17.1629 62.3076 16.2668 61.5247 16.75Z\" fill=\"#2379F4\"/>\n<path d=\"M95.8159 40.165C94.4289 39.1165 92.4854 38.4267 89.9854 38.0955L85.3776 37.5032C84.1413 37.3497 83.1926 37.0686 82.5317 36.6599C82.2184 36.4813 81.96 36.2193 81.7847 35.9027C81.6094 35.586 81.5239 35.2269 81.5377 34.8647C81.5377 33.0125 83.2842 32.0873 86.7774 32.089C88.7633 32.0612 90.7381 32.3924 92.6079 33.0668C93.1817 33.2698 93.7341 33.5297 94.2569 33.8429C94.5363 34.0081 94.8002 34.1986 95.0454 34.412C95.193 34.3457 95.3305 34.2586 95.4538 34.1533C95.6458 34.0031 95.8183 33.8293 95.9675 33.6359C96.1418 33.4085 96.2932 33.1641 96.4195 32.9065C96.5609 32.6088 96.6305 32.2818 96.6224 31.9519C96.6224 30.8293 95.7569 29.9187 94.0257 29.2203C92.2945 28.5219 89.8622 28.177 86.7286 28.1856C85.1211 28.1527 83.5168 28.3442 81.9615 28.7547C80.8055 29.0563 79.7221 29.5904 78.7765 30.3249C78.0025 30.9337 77.3857 31.7217 76.9786 32.6219C76.6008 33.4894 76.4083 34.4271 76.4135 35.3743C76.3859 36.3727 76.5955 37.3633 77.0248 38.2637C77.4257 39.0618 78.0051 39.7551 78.7175 40.2891C79.5047 40.8658 80.3837 41.3032 81.3168 41.5825C82.3984 41.9165 83.5113 42.1375 84.6378 42.2422L88.1618 42.6535C89.7885 42.8259 90.9178 43.1363 91.5497 43.5847C91.8591 43.7984 92.1087 44.0885 92.2747 44.4274C92.4407 44.7662 92.5174 45.1423 92.4974 45.5196C92.4974 46.5216 92.0009 47.2269 91.0077 47.6356C90.0146 48.0443 88.7303 48.2487 87.155 48.2487C86.0363 48.2553 84.9191 48.1645 83.8159 47.9771C82.8916 47.818 81.9786 47.5976 81.0831 47.3174C80.4 47.1024 79.7287 46.8511 79.0719 46.5647C78.6781 46.4034 78.2997 46.206 77.9418 45.9749L76 49.4774C76.3686 49.7424 76.7617 49.971 77.1738 50.1603C77.9419 50.528 78.735 50.84 79.5471 51.0941C80.6793 51.4497 81.8332 51.7305 83.0017 51.9348C84.4639 52.1895 85.9459 52.3116 87.4298 52.2996C90.8014 52.2996 93.3844 51.6167 95.1789 50.2508C96.0362 49.6313 96.7296 48.8096 97.1985 47.8576C97.6673 46.9056 97.8972 45.8524 97.8682 44.7901C97.8853 42.7552 97.2012 41.2135 95.8159 40.165Z\" fill=\"white\"/>\n<path d=\"M106.298 18.1178C105.425 18.1178 104.757 18.317 104.264 18.7102C103.771 19.1034 103.542 19.7113 103.542 20.5209V51.6555H108.692V18.6197C108.54 18.5602 108.24 18.4593 107.788 18.3248C107.304 18.1826 106.802 18.1128 106.298 18.1178Z\" fill=\"white\"/>\n<path d=\"M118.198 18.5731C117.787 18.5682 117.379 18.6462 116.999 18.8023C116.618 18.9583 116.273 19.1895 115.982 19.4821C115.692 19.7746 115.462 20.1228 115.307 20.506C115.152 20.8892 115.075 21.2997 115.08 21.7135C115.082 22.2288 115.211 22.7356 115.454 23.1891C115.697 23.6426 116.047 24.0289 116.473 24.3139C116.9 24.5989 117.389 24.7739 117.899 24.8234C118.408 24.8728 118.921 24.7953 119.394 24.5976C119.867 24.3999 120.284 24.0881 120.608 23.6897C120.933 23.2913 121.155 22.8186 121.255 22.3132C121.356 21.8079 121.331 21.2854 121.183 20.7919C121.036 20.2985 120.77 19.8492 120.409 19.4836C120.122 19.1879 119.777 18.9546 119.397 18.798C119.017 18.6414 118.609 18.5648 118.198 18.5731V18.5731Z\" fill=\"white\"/>\n<path d=\"M118.37 28.8168C117.497 28.8168 116.829 29.0082 116.336 29.3858C115.843 29.7635 115.622 30.3688 115.622 31.2225V51.658H120.771V29.3134C120.62 29.2513 120.319 29.153 119.867 29.0159C119.38 28.877 118.876 28.8099 118.37 28.8168Z\" fill=\"white\"/>\n<path d=\"M146.384 30.7465C145.354 29.8828 144.164 29.2331 142.884 28.8349C141.451 28.3836 139.957 28.161 138.456 28.1752C136.962 28.1596 135.475 28.3822 134.051 28.8349C132.777 29.2366 131.595 29.8861 130.57 30.7465C129.588 31.5801 128.799 32.6202 128.259 33.7937C127.687 35.0744 127.405 36.4667 127.432 37.8705V51.658H132.581V39.1897C132.581 37.1203 133.071 35.5501 134.051 34.4792C135.03 33.4083 136.498 32.8702 138.456 32.8651C140.382 32.8651 141.842 33.4031 142.837 34.4792C143.832 35.5553 144.33 37.1255 144.33 39.1897V51.658H149.479V37.8705C149.507 36.4693 149.234 35.0786 148.678 33.7937C148.15 32.6177 147.366 31.5763 146.384 30.7465V30.7465Z\" fill=\"white\"/>\n<path d=\"M168.409 46.5129C167.903 46.8416 167.357 47.1025 166.783 47.289C166.001 47.5667 165.176 47.7059 164.346 47.7003C163.821 47.7026 163.298 47.6419 162.787 47.5192C162.309 47.4066 161.862 47.1893 161.477 46.8828C161.087 46.5612 160.784 46.1457 160.596 45.6748C160.365 45.058 160.258 44.4013 160.28 43.7425V33.5894H168.954C169.092 33.2861 169.213 32.9752 169.316 32.6581C169.469 32.1954 169.546 31.7103 169.542 31.2225C169.542 30.4464 169.339 29.8359 168.931 29.4272C168.522 29.0185 167.821 28.8168 166.827 28.8168H160.277V22.8672C160.126 22.8051 159.818 22.7068 159.35 22.5697C158.858 22.4308 158.349 22.362 157.837 22.3653C157.13 22.3252 156.43 22.5256 155.849 22.9344C155.356 23.2991 155.125 23.9019 155.125 24.7633V44.7824C155.125 47.0243 155.803 48.835 157.159 50.2146C158.515 51.5942 160.519 52.284 163.17 52.284C164.032 52.2871 164.893 52.2004 165.738 52.0254C166.46 51.8751 167.17 51.6701 167.862 51.4123C168.419 51.2043 168.956 50.9445 169.465 50.6363C169.902 50.3621 170.244 50.1344 170.492 49.9534L168.409 46.5129Z\" fill=\"white\"/>\n<path d=\"M40.472 78L41.6458 73.9195H39.559V72.8388H41.9439L42.7265 70.1185H40.4906V69.0379H43.0246L44.1984 64.9574H45.4654L44.2916 69.0379H47.9808L49.1546 64.9574H50.4216L49.2478 69.0379H51.3346V70.1185H48.9497L48.1671 72.8388H50.403V73.9195H47.869L46.6951 78H45.4282L46.602 73.9195H42.9128L41.739 78H40.472ZM43.2109 72.8388H46.9001L47.6827 70.1185H43.9935L43.2109 72.8388ZM59.7822 77.0684L54.6769 66.6157V78H53.3354V64.9574H55.3477L59.8008 74.2176L64.2353 64.9574H66.2476V78H64.8874V66.597L59.7822 77.0684ZM71.6121 78.1677C70.5935 78.1677 69.7613 77.9068 69.1154 77.3851C68.4819 76.8634 68.1651 76.1865 68.1651 75.3542C68.1651 74.4847 68.5005 73.7953 69.1713 73.286C69.842 72.7643 70.7488 72.5035 71.8916 72.5035C72.3884 72.5035 72.8667 72.5594 73.3263 72.6712C73.7983 72.7829 74.233 72.9382 74.6305 73.137V71.9072C74.6305 71.1619 74.4194 70.603 73.997 70.2303C73.5747 69.8453 72.9598 69.6527 72.1524 69.6527C71.6928 69.6527 71.227 69.721 70.755 69.8577C70.2954 69.9819 69.7861 70.1868 69.2272 70.4725L68.7055 69.4105C69.3762 69.0875 69.9973 68.8515 70.5687 68.7025C71.1401 68.5534 71.7115 68.4789 72.2829 68.4789C73.4629 68.4789 74.3697 68.7584 75.0032 69.3173C75.6491 69.8763 75.9721 70.6775 75.9721 71.7209V78H74.6305V77.087C74.2082 77.4472 73.7424 77.7205 73.2331 77.9068C72.7362 78.0807 72.1959 78.1677 71.6121 78.1677ZM69.488 75.3169C69.488 75.8387 69.7054 76.2672 70.1401 76.6026C70.5873 76.9255 71.1587 77.087 71.8543 77.087C72.4133 77.087 72.9226 77.0001 73.3822 76.8262C73.8418 76.6523 74.2579 76.379 74.6305 76.0063V74.2549C74.2455 73.9941 73.8293 73.8077 73.3822 73.6959C72.9474 73.5717 72.463 73.5096 71.9288 73.5096C71.1836 73.5096 70.5873 73.6773 70.1401 74.0127C69.7054 74.3356 69.488 74.7704 69.488 75.3169ZM85.5607 78V77.0125C85.1632 77.3851 84.7036 77.6708 84.1819 77.8696C83.6726 78.0559 83.1261 78.1491 82.5423 78.1491C81.8839 78.1491 81.2691 78.0248 80.6977 77.7764C80.1263 77.528 79.6294 77.1926 79.2071 76.7703C78.7848 76.3355 78.4494 75.82 78.2009 75.2238C77.9649 74.6275 77.8469 73.9941 77.8469 73.3233C77.8469 72.6525 77.9649 72.0252 78.2009 71.4414C78.4494 70.8452 78.7848 70.3359 79.2071 69.9136C79.6294 69.4788 80.1263 69.1372 80.6977 68.8888C81.2815 68.6404 81.9026 68.5161 82.5609 68.5161C83.095 68.5161 83.6167 68.6093 84.126 68.7956C84.6477 68.9695 85.1198 69.2304 85.5421 69.5782V64.9574L86.9209 64.6406V78H85.5607ZM79.2071 73.3047C79.2071 73.8139 79.294 74.2922 79.4679 74.7393C79.6543 75.1865 79.9027 75.5716 80.2132 75.8945C80.5238 76.2175 80.8902 76.4721 81.3125 76.6585C81.7349 76.8448 82.1883 76.938 82.6727 76.938C83.2565 76.938 83.7969 76.8262 84.2937 76.6026C84.803 76.3666 85.2191 76.0436 85.5421 75.6337V71.0129C85.2191 70.6154 84.803 70.3049 84.2937 70.0813C83.7844 69.8453 83.2441 69.7272 82.6727 69.7272C81.7038 69.7272 80.884 70.0688 80.2132 70.752C79.5425 71.4352 79.2071 72.2861 79.2071 73.3047ZM97.1285 77.0125C96.6068 77.3976 96.0602 77.6895 95.4888 77.8882C94.9299 78.0745 94.315 78.1677 93.6442 78.1677C92.9735 78.1677 92.34 78.0435 91.7437 77.795C91.1599 77.5466 90.6506 77.205 90.2159 76.7703C89.7936 76.3355 89.4582 75.8262 89.2097 75.2424C88.9613 74.6462 88.8371 74.0065 88.8371 73.3233C88.8371 72.6525 88.9551 72.0252 89.1911 71.4414C89.4395 70.8576 89.7687 70.3483 90.1786 69.9136C90.5885 69.4788 91.073 69.1372 91.6319 68.8888C92.2033 68.6404 92.8058 68.5161 93.4393 68.5161C94.0604 68.5161 94.638 68.6404 95.1721 68.8888C95.7186 69.1372 96.1844 69.4788 96.5695 69.9136C96.967 70.3483 97.2775 70.8576 97.5011 71.4414C97.7371 72.0252 97.8551 72.6525 97.8551 73.3233V73.7518H90.1973C90.2966 74.671 90.6755 75.435 91.3338 76.0436C91.9922 76.6523 92.7809 76.9566 93.7001 76.9566C94.1846 76.9566 94.6566 76.8821 95.1162 76.733C95.5758 76.5839 95.9609 76.3728 96.2714 76.0995L97.1285 77.0125ZM93.402 69.7272C92.5946 69.7272 91.8928 70.0005 91.2966 70.5471C90.7128 71.0936 90.3587 71.7954 90.2345 72.6525H96.4763C96.3521 71.8327 95.9981 71.1433 95.4143 70.5843C94.8429 70.0129 94.1722 69.7272 93.402 69.7272ZM103.528 78H102.074L99.261 64.9574H100.714L102.932 75.7269L106.155 64.9574H107.552L110.776 75.6896L113.012 64.9574H114.409L111.596 78H110.142L106.844 66.951L103.528 78ZM116.903 66.8765C116.654 66.8765 116.437 66.7833 116.251 66.597C116.064 66.4107 115.971 66.1933 115.971 65.9449C115.971 65.684 116.064 65.4667 116.251 65.2928C116.437 65.1064 116.654 65.0133 116.903 65.0133C117.164 65.0133 117.381 65.1064 117.555 65.2928C117.741 65.4667 117.834 65.684 117.834 65.9449C117.834 66.1933 117.741 66.4107 117.555 66.597C117.381 66.7833 117.164 66.8765 116.903 66.8765ZM117.592 68.6652V78H116.213V68.6652H117.592ZM121.099 75.9132V69.839H119.086V68.6652H121.099V66.2989L122.459 65.9449V68.6652H125.272V69.839H122.459V75.5592C122.459 76.056 122.571 76.4163 122.794 76.6398C123.018 76.851 123.384 76.9566 123.894 76.9566C124.154 76.9566 124.384 76.938 124.583 76.9007C124.794 76.8634 125.018 76.8013 125.254 76.7144V77.9255C125.018 78.0124 124.751 78.0745 124.452 78.1118C124.167 78.1491 123.887 78.1677 123.614 78.1677C122.807 78.1677 122.186 77.9752 121.751 77.5901C121.316 77.205 121.099 76.6461 121.099 75.9132ZM126.749 78V64.9574L128.127 64.6406V69.7645C128.488 69.3422 128.916 69.0254 129.413 68.8143C129.91 68.5907 130.463 68.4789 131.071 68.4789C132.127 68.4789 132.99 68.8143 133.661 69.485C134.332 70.1558 134.667 71.0253 134.667 72.0936V78H133.307V72.373C133.307 71.5532 133.071 70.9011 132.599 70.4166C132.127 69.9322 131.494 69.69 130.699 69.69C130.14 69.69 129.637 69.8142 129.189 70.0626C128.755 70.3111 128.401 70.6651 128.127 71.1247V78H126.749ZM136.006 75.7641L136.956 74.7766C137.627 75.4846 138.341 76.025 139.099 76.3976C139.869 76.7578 140.676 76.938 141.521 76.938C142.539 76.938 143.378 76.6957 144.036 76.2113C144.695 75.7269 145.024 75.112 145.024 74.3667C145.024 73.6959 144.788 73.1804 144.316 72.8202C143.856 72.46 143.092 72.1929 142.024 72.019L140.142 71.7209C138.887 71.5222 137.962 71.1557 137.366 70.6216C136.77 70.0751 136.471 69.3422 136.471 68.423C136.471 67.3423 136.894 66.4666 137.738 65.7958C138.583 65.1251 139.689 64.7897 141.055 64.7897C141.962 64.7897 142.862 64.9387 143.757 65.2369C144.651 65.535 145.465 65.9635 146.198 66.5225L145.378 67.6404C144.67 67.1063 143.949 66.7026 143.216 66.4293C142.484 66.1561 141.744 66.0194 140.999 66.0194C140.068 66.0194 139.31 66.2368 138.726 66.6715C138.155 67.0939 137.869 67.6404 137.869 68.3112C137.869 68.9198 138.074 69.3857 138.484 69.7086C138.894 70.0316 139.571 70.2676 140.515 70.4166L142.378 70.7148C143.794 70.9508 144.825 71.3545 145.471 71.9259C146.129 72.4973 146.458 73.286 146.458 74.2922C146.458 74.8636 146.334 75.3915 146.086 75.8759C145.85 76.3479 145.508 76.7578 145.061 77.1056C144.626 77.441 144.098 77.7081 143.477 77.9068C142.869 78.0932 142.198 78.1863 141.465 78.1863C140.434 78.1863 139.44 77.9814 138.484 77.5715C137.54 77.1491 136.714 76.5467 136.006 75.7641ZM149.525 64.6406V78H148.147V64.9574L149.525 64.6406ZM152.566 66.8765C152.318 66.8765 152.1 66.7833 151.914 66.597C151.728 66.4107 151.635 66.1933 151.635 65.9449C151.635 65.684 151.728 65.4667 151.914 65.2928C152.1 65.1064 152.318 65.0133 152.566 65.0133C152.827 65.0133 153.044 65.1064 153.218 65.2928C153.405 65.4667 153.498 65.684 153.498 65.9449C153.498 66.1933 153.405 66.4107 153.218 66.597C153.044 66.7833 152.827 66.8765 152.566 66.8765ZM153.256 68.6652V78H151.877V68.6652H153.256ZM155.607 78V68.6652H156.986V69.7645C157.346 69.3422 157.774 69.0254 158.271 68.8143C158.768 68.5907 159.321 68.4789 159.93 68.4789C160.985 68.4789 161.849 68.8143 162.519 69.485C163.19 70.1558 163.526 71.0253 163.526 72.0936V78H162.165V72.373C162.165 71.5532 161.929 70.9011 161.457 70.4166C160.985 69.9322 160.352 69.69 159.557 69.69C158.998 69.69 158.495 69.8142 158.048 70.0626C157.613 70.3111 157.259 70.6651 156.986 71.1247V78H155.607ZM166.624 75.9132V69.839H164.612V68.6652H166.624V66.2989L167.984 65.9449V68.6652H170.798V69.839H167.984V75.5592C167.984 76.056 168.096 76.4163 168.32 76.6398C168.543 76.851 168.91 76.9566 169.419 76.9566C169.68 76.9566 169.91 76.938 170.108 76.9007C170.32 76.8634 170.543 76.8013 170.779 76.7144V77.9255C170.543 78.0124 170.276 78.0745 169.978 78.1118C169.692 78.1491 169.413 78.1677 169.139 78.1677C168.332 78.1677 167.711 77.9752 167.276 77.5901C166.842 77.205 166.624 76.6461 166.624 75.9132Z\" fill=\"white\"/>\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_3 : & 'static [u8] = b"<svg width=\"8\" height=\"6\" viewBox=\"0 0 8 6\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 1C0 0.864583 0.0260417 0.735677 0.078125 0.613281C0.130208 0.490885 0.200521 0.385417 0.289062 0.296875C0.380208 0.205729 0.485677 0.134115 0.605469 0.0820312C0.72526 0.0273438 0.854167 0 0.992188 0H7.01172C7.14714 0 7.27474 0.0260417 7.39453 0.078125C7.51693 0.130208 7.6224 0.201823 7.71094 0.292969C7.79948 0.384115 7.86979 0.490885 7.92188 0.613281C7.97396 0.733073 8 0.860677 8 0.996094C8 1.10547 7.98438 1.20573 7.95312 1.29688C7.92448 1.38802 7.88021 1.47917 7.82031 1.57031L5.21875 5.35547C5.08073 5.55599 4.90365 5.71354 4.6875 5.82812C4.47396 5.94271 4.24479 6 4 6C3.75521 6 3.52474 5.94271 3.30859 5.82812C3.09505 5.71354 2.91927 5.55599 2.78125 5.35547L0.179688 1.57031C0.119792 1.48177 0.0742188 1.39193 0.0429688 1.30078C0.0143229 1.20964 0 1.10938 0 1Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_19 : & 'static [u8] = b"<svg width=\"6\" height=\"8\" viewBox=\"0 0 6 8\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 4C0 3.75521 0.0572917 3.52604 0.171875 3.3125C0.286458 3.09635 0.44401 2.91927 0.644531 2.78125L4.42969 0.179687C4.51823 0.119792 4.60807 0.0755208 4.69922 0.046875C4.79036 0.015625 4.89062 0 5 0C5.13542 0 5.26432 0.0260417 5.38672 0.078125C5.50911 0.130208 5.61458 0.201823 5.70312 0.292969C5.79427 0.38151 5.86589 0.485677 5.91797 0.605469C5.97266 0.72526 6 0.854167 6 0.992187L6 7.00781C6 7.14583 5.97266 7.27474 5.91797 7.39453C5.86589 7.51432 5.79427 7.61979 5.70313 7.71094C5.61458 7.79948 5.50911 7.86979 5.38672 7.92187C5.26432 7.97396 5.13542 8 5 8C4.79427 8 4.60417 7.9401 4.42969 7.82031L0.644531 5.21875C0.44401 5.08073 0.286458 4.90495 0.171875 4.69141C0.0572917 4.47526 0 4.24479 0 4Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_4 : & 'static [u8] = b"<svg width=\"6\" height=\"8\" viewBox=\"0 0 6 8\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0 7.00781L0 0.992187C0 0.854167 0.0260417 0.72526 0.078125 0.605469C0.132812 0.485677 0.204427 0.38151 0.292969 0.292969C0.384115 0.201823 0.489583 0.130208 0.609375 0.078125C0.731771 0.0260417 0.861979 0 1 0C1.20573 0 1.39583 0.0598958 1.57031 0.179687L5.35547 2.78125C5.55859 2.92187 5.71615 3.09896 5.82813 3.3125C5.94271 3.52604 6 3.75521 6 4C6 4.24479 5.94271 4.47396 5.82813 4.6875C5.71615 4.90104 5.55859 5.07812 5.35547 5.21875L1.57031 7.82031C1.39583 7.9401 1.20573 8 1 8C0.861979 8 0.731771 7.97396 0.609375 7.92188C0.489583 7.86979 0.384115 7.79948 0.292969 7.71094C0.204427 7.61979 0.132813 7.51432 0.078125 7.39453C0.0260417 7.27474 0 7.14583 0 7.00781Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     static SLINT_EMBEDDED_RESOURCE_18 : & 'static [u8] = b"<svg width=\"8\" height=\"6\" viewBox=\"0 0 8 6\" fill=\"none\" xmlns=\"http://www.w3.org/2000/svg\">\n<path d=\"M0.992188 6C0.854167 6 0.72526 5.97396 0.605469 5.92188C0.485677 5.86719 0.380208 5.79557 0.289062 5.70703C0.200521 5.61589 0.130208 5.51042 0.078125 5.39062C0.0260417 5.26823 0 5.13802 0 5C0 4.89062 0.0143229 4.79036 0.0429688 4.69922C0.0742188 4.60807 0.119792 4.51823 0.179688 4.42969L2.78125 0.644531C2.84896 0.545573 2.92839 0.458333 3.01953 0.382812C3.11068 0.307292 3.20833 0.244792 3.3125 0.195312C3.41927 0.143229 3.53125 0.104167 3.64844 0.078125C3.76562 0.0520833 3.88281 0.0390625 4 0.0390625C4.11719 0.0390625 4.23438 0.0520833 4.35156 0.078125C4.46875 0.104167 4.57943 0.143229 4.68359 0.195312C4.79036 0.244792 4.88932 0.307292 4.98047 0.382812C5.07161 0.458333 5.15104 0.545573 5.21875 0.644531L7.82031 4.42969C7.88021 4.51823 7.92448 4.60807 7.95312 4.69922C7.98438 4.79036 8 4.89062 8 5C8 5.13802 7.97396 5.26823 7.92188 5.39062C7.86979 5.51042 7.79948 5.61589 7.71094 5.70703C7.6224 5.79557 7.51693 5.86719 7.39453 5.92188C7.27474 5.97396 7.14714 6 7.01172 6H0.992188Z\" fill=\"white\" fill-opacity=\"0.5442\" />\n</svg>\n" ;
     }
 # [allow (unused_imports)] pub use slint_generatedMainWindow :: {
     r#AboutDialog , r#MainWindow , r#GameState , r#Position , r#Tile , }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
