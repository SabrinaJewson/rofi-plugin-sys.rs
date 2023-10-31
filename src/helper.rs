//! Helper utilities.
//!
//! This corresponds to `helper.h`.

use {
    crate::{RofiIntMatcher, RofiRangePair},
    ::std::os::raw::{c_char, c_int, c_long, c_uint},
};

#[cfg(any(doc, rofi_next))]
use crate::{ConfigEntry, Property, PropertyType};

extern "C" {
    /// Parses a string into arguments, replacing keys with values.
    ///
    /// Returns true if successful, false if it failed.
    ///
    /// - `string`: The input string.
    /// - `output`: Pointer to 2-dimensional array with parsed string.
    /// - `length`: Length of 2-dimensional array.
    /// - `...`: Key, value parse. Replace the string key with value.
    #[link_name = "helper_parse_setup"]
    pub fn parse_setup(string: *mut c_char, output: *mut *mut *mut c_char, length: *mut c_int, ...);

    /// Tokenize the string on spaces.
    ///
    /// Returns a newly allocated array of matching objects.
    #[link_name = "helper_tokenize"]
    pub fn tokenize(input: *const c_char, case_sensitive: c_int) -> *mut *mut RofiIntMatcher;

    /// Frees the array of matching objects.
    #[link_name = "helper_tokenize_free"]
    pub fn tokenize_free(tokens: *mut *mut RofiIntMatcher);

    /// Parse command line argument `key` to character.
    /// This supports character escaping.
    ///
    /// Returns true if the key was found and `val` was set.
    ///
    /// - `key`: The key to search for.
    /// - `val`: Pointer to the string to set to the key value (if found).
    pub fn find_arg_char(key: *const c_char, val: *mut c_char) -> c_int;

    /// Parse command line argument `key` to unsigned int.
    ///
    /// Returns true if key was found and `val` was set.
    ///
    /// - `key`: The key to search for.
    /// - `val`: Pointer to the string to set to the key value (if found).
    pub fn find_arg_uint(key: *const c_char, val: *mut c_uint) -> c_int;

    /// Parse command line argument `key` to int.
    ///
    /// Returns true if key was found and `val` was set.
    ///
    /// - `key`: The key to search for.
    /// - `val`: Pointer to the string to set to the key value (if found).
    pub fn find_arg_int(key: *const c_char, val: *mut c_int) -> c_int;

    /// Parse command line argument `key` to string.
    ///
    /// Returns true if key was found and `val` was set.
    ///
    /// - `key`: The key to search for.
    /// - `val`: Pointer to the string to set to the key value (if found).
    pub fn find_arg_str(key: *const c_char, val: *mut *mut c_char) -> c_int;

    /// Parse all command line options `key` to string vector.
    ///
    /// Returns a string vector which the user must free.
    pub fn find_arg_strv(key: *const c_char) -> *mut *const c_char;

    /// Check if key is passed as argument.
    ///
    /// Returns position of string or -1 if not found.
    pub fn find_arg(key: *const c_char) -> c_int;

    /// Tokenized match, match tokens to line input.
    ///
    /// Returns true when matches, false otherwise.
    ///
    /// - `tokens`: List of input tokens to match.
    /// - `input`: The entry to match against.
    #[link_name = "helper_token_match"]
    pub fn token_match(tokens: *const *mut RofiIntMatcher, input: *const c_char) -> c_int;

    /// Execute cmd using `config.run_command` and outputs the result (stdout) to the opened file
    /// descriptor.
    ///
    /// Returns a valid file descriptor on success, or -1 on failure.
    pub fn execute_generator(cmd: *const c_char) -> c_int;

    /// Returns file descriptor (or -1 when failed).
    ///
    /// - `pidfile`: The pidfile to create.
    /// - `kill`: Try killing running instance.
    pub fn create_pid_file(pidfile: *const c_char, kill: glib_sys::gboolean) -> c_int;

    /// Remove pid file.
    pub fn remove_pid_file(fd: c_int);

    /// Do some input validation, especially the first few could break things. It is good to catch
    /// them beforehand.
    ///
    /// This function exits the program with 1 when it finds an invalid configuration.
    pub fn config_sanity_check() -> c_int;

    /// Parses a string into a character.
    #[link_name = "helper_parse_char"]
    pub fn parse_char(arg: *const c_char) -> c_char;

    /// Set the application arguments.
    pub fn cmd_set_arguments(argc: c_int, argv: *mut *mut c_char);

    /// Expand path, both `~` and `~<user>`.
    #[link_name = "rofi_expand_path"]
    pub fn expand_path(input: *const c_char) -> *const c_char;

    /// UTF-8 aware levenshtein distance calculation.
    ///
    /// - `needle`: The string to match weight off
    /// - `needlelen`: The length of the needle
    /// - `haystack`: The string to match against
    /// - `haystacklen`: The length of the haystack
    pub fn levenshtein(
        needle: *const c_char,
        needlelen: c_long,
        haystack: *const c_char,
        haystacklen: c_long,
    ) -> c_uint;

    /// Convert string to valid UTF-8, replacing invalid parts with replacement character.
    ///
    /// - `data`: The unvalidated character array holding possible UTF-8 data
    /// - `length`: The length of `data`
    #[link_name = "rofi_force_utf8"]
    pub fn force_utf8(data: *const c_char, length: isize) -> *const c_char;

    /// Converts latin to UTF-8.
    #[link_name = "rofi_latin_to_utf8_strdup"]
    pub fn latin_to_utf8_strdup(input: *const c_char, length: isize) -> *const c_char;

    /// Run Rofi's global sequence alignment algorithm to find the maximum accumulated score by
    /// aligning `pattern` to `str`.
    /// It applies when `pattern` is a subsequence of `str`.
    ///
    ///  Scoring criteria
    ///  - Prefer matches at the start of a word, or the start of subwords in
    ///  CamelCase/camelCase/camel123 words. See WORD_START_SCORE/CAMEL_SCORE.
    ///  - Non-word characters matter. See NON_WORD_SCORE.
    ///  - The first characters of words of `pattern` receive bonus because they usually have more
    ///  significance than the rest. See PATTERN_START_MULTIPLIER/PATTERN_NON_START_MULTIPLIER.
    ///  - Superfluous characters in `str` will reduce the score (gap penalty). See GAP_SCORE.
    ///  - Prefer early occurrence of the first character. See LEADING_GAP_SCORE/GAP_SCORE.
    ///
    ///  The recurrence of the dynamic programming:
    ///  - `dp[i][j]`: maximum accumulated score by aligning `pattern[0..i]` to `str[0..j]`
    ///  - `dp[0][j] = leading_gap_penalty(0, j) + score[j]`
    ///  - `dp[i][j] = max(dp[i-1][j-1] + CONSECUTIVE_SCORE, max(dp[i-1][k] + gap_penalty(k+1, j) + score[j] : k < j))`
    ///
    /// The first dimension can be suppressed since we do not need a matching scheme, which reduces
    /// the space complexity from O(N*M) to O(M)
    ///
    /// Returns the sorting weight.
    ///
    /// - `pattern`: The user input to match against.
    /// - `plen`: The length of `pattern`.
    /// - `str`: The input to match against `pattern`.
    /// - `slen`: Length of `str`.
    #[link_name = "rofi_scorer_fuzzy_evaluate"]
    pub fn scorer_fuzzy_evaluate(
        pattern: *const c_char,
        plen: c_long,
        str: *const c_char,
        slen: c_long,
    ) -> c_int;

    /// Compares the `G_NORMALIZE_ALL_COMPOSE` forms of the two strings.
    ///
    /// Returns less than, equal to, or greater than zero if the first `n` characters (not bytes) of
    /// `a` are found, respectively, to be less than, to match, or be greater than the first `n`
    /// characters (not bytes) of `b`.
    ///
    /// - `a`: First UTF-8 string to compare, non-null.
    /// - `b`: Second UTF-8 string to compare, non-null.
    /// - `n`: Maximum number of characters to compare.
    pub fn utf8_strncmp(a: *const c_char, b: *const c_char, n: usize) -> c_int;
}

/// The startup notification context of the application to launch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RofiHelperExecuteContext {
    /// The name of the application.
    pub name: *const c_char,
    /// The binary name of the application.
    pub binary: *const c_char,
    /// The description of the launch.
    pub description: *const c_char,
    /// The icon name of the application.
    pub icon: *const c_char,
    /// The application ID (desktop file with the `.desktop` suffix).
    pub app_id: *const c_char,
    /// The window manager class of the application.
    pub wmclass: *const c_char,
    /// The command we run.
    pub command: *const c_char,
}

extern "C" {
    /// Executes the comand.
    ///
    /// Returns true when successful, false otherwise.
    ///
    /// - `wd`: The working directory.
    /// - `args`: The argments of the command to exec.
    /// - `error_precmd`: Prefix to error message command.
    /// - `error_cmd`: Error message command.
    /// - `context`: The startup notification context, if any.
    #[link_name = "helper_execute"]
    pub fn execute(
        wd: *const c_char,
        args: *mut *mut c_char,
        error_precmd: *const c_char,
        error_cmd: *const c_char,
        context: *mut RofiHelperExecuteContext,
    ) -> glib_sys::gboolean;

    /// Executes the comand.
    /// If needed memebrs of `context` are null, they will be filled.
    ///
    /// Returns true when successful, false otherwise.
    ///
    /// - `wd`: The working directory (optional).
    /// - `cmd`: The command to execute.
    /// - `run_in_term`: Indicates if the command should be run in a terminal.
    /// - `context`: The startup notification context, if any.
    #[link_name = "helper_execute_command"]
    pub fn execute_command(
        wd: *const c_char,
        cmd: *const c_char,
        run_in_term: glib_sys::gboolean,
        context: *mut RofiHelperExecuteContext,
    ) -> glib_sys::gboolean;

    /// Get a cairo surface from an SVG path.
    ///
    /// - `param`: The file path.
    /// - `height`: The wanted height.
    pub fn cairo_image_surface_create_from_svg(
        file: *const c_char,
        height: c_int,
    ) -> *mut cairo_sys::cairo_surface_t;

    /// Parse ranges.
    ///
    /// - `input`: String to parse.
    /// - `list`: List of ranges.
    /// - `length`: Length of list.
    pub fn parse_ranges(input: *mut c_char, list: *mut *mut RofiRangePair, length: *mut c_uint);

    /// This functions outputs the formatted string to stdout, appends a newline (`\n`)
    /// character and calls flush on the file descriptor.
    ///
    /// - `format`: The format string used. See below for possible syntax.
    /// - `string`: The selected entry.
    /// - `selected_line`: The selected line index.
    /// - `filter`: The entered filter.
    ///
    /// Currently the following formats are supported:
    /// - i: Print the index (0-(N-1))
    /// - d: Print the index (1-N)
    /// - s: Print input string.
    /// - q: Print quoted input string.
    /// - f: Print the entered filter.
    /// - F: Print the entered filter, quoted
    #[link_name = "rofi_output_formatted_line"]
    pub fn output_formatted_line(
        format: *const c_char,
        string: *const c_char,
        selected_line: c_int,
        filter: *const c_char,
    );

    /// Items {key} are replaced by the value if `{key}` is passed as key/value pair, otherwise
    /// removed from string. If the {key} is in between [] all the text between [] are removed if
    /// {key} is not found. Otherwise key is replaced and [ & ] removed.
    ///
    /// This allows for optional replacement, e.g. `{ssh-client} [-t {title}] -e "{cmd}"` the `-t
    /// {title}` is only there if {title} is set.
    ///
    /// Returns a new string with the keys replaced.
    ///
    /// - `string`: The string with elements to be replaced.
    /// - `...`: Set of {key}, value that will be replaced, terminated by a null.
    #[link_name = "helper_string_replace_if_exists"]
    pub fn string_replace_if_exists(string: *mut c_char, ...) -> *mut c_char;

    /// Returns path to theme or copy of filename if not found.
    ///
    /// - `file`: File name passed to option.
    /// - `ext`: File extension passed to option.
    ///
    /// **When `cfg(rofi_next)` is enabled**:
    /// - `ext` is of type `*const *const c_char` and is a null-terminated array of file extensions.
    /// - The function returns a `NonNull<c_char>`.
    #[link_name = "helper_get_theme_path"]
    #[cfg(not(rofi_next))]
    pub fn get_theme_path(file: *const c_char, ext: *const c_char) -> *mut c_char;
    #[cfg(rofi_next)]
    pub fn get_theme_path(file: *const c_char, ext: *const *const c_char) -> std::ptr::NonNull<c_char>;

    /// Find the configuration element.
    /// If not exact, the closest specified element is returned.
    /// Returns the [`ThemeWidget`](crate::ThemeWidget) if found, otherwise null.
    ///
    /// - `name`: The name of the element to find.
    /// - `state`: The state of the element.
    /// - `exact`: If the match should be exact, or the parent can be included.
    ///
    /// **Semver-exempt and only available with `cfg(rofi_next)`.**
    #[cfg(any(doc, rofi_next))]
    #[link_name = "rofi_config_find_widget"]
    pub fn config_find_widget(
        name: *const c_char,
        state: *const c_char,
        exact: glib_sys::gboolean,
    ) -> *mut ConfigEntry;

    /// Find the property on the widget.
    /// If not exact, the parents are searched recursively until match is found.
    /// Returns the [`Property`] if found, otherwise null.
    ///
    /// - `widget`: The widget to find the property on.
    /// - `type`: The [`PropertyType`] to find.
    /// - `property`: The property to find.
    /// - `exact`: If the property should only be found on this widget, or on parents if not found.
    ///
    /// **Semver-exempt and only available with `cfg(rofi_next)`.**
    #[cfg(any(doc, rofi_next))]
    #[link_name = "rofi_theme_find_property"]
    pub fn theme_find_property(
        widget: *mut ConfigEntry,
        r#type: PropertyType,
        property: *const c_char,
        exact: glib_sys::gboolean,
    ) -> *mut Property;
}
