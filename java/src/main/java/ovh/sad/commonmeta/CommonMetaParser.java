package ovh.sad.commonmeta;

import com.google.gson.Gson;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class CommonMetaParser {

    // Load the native library
    static {
        try {
            NativeUtils.loadLibraryFromResources("commonmeta");
        } catch (IOException e) {
            throw new RuntimeException(e);
        }
    }

    public static class ParseResult {
        public Map<String, String> pairs;
        public boolean success;
        public String error;

        public ParseResult() {
            this.pairs = new HashMap<>();
        }

        public ParseResult(Map<String, String> pairs, boolean success, String error) {
            this.pairs = pairs;
            this.success = success;
            this.error = error;
        }
    }

    // Native methods implemented in Rust
    private static native String parseToJson(String input);
    private static native Map<String, String> parse(String input);
    private static native boolean validate(String input);
    private static native String getError(String input);

    // Java convenience methods
    public static ParseResult parseWithResult(String input) {
        try {
            return new Gson().fromJson(parseToJson(input), ParseResult.class);
        } catch (Exception e) {
            return new ParseResult(new HashMap<>(), false, e.getMessage());
        }
    }

    public static Map<String, String> parsePairs(String input) {
        Map<String, String> result = parse(input);
        return result != null ? result : new HashMap<>();
    }

    public static boolean isValid(String input) {
        return validate(input);
    }

    public static String getErrorMessage(String input) {
        return getError(input);
    }

    public static String toJson(String input) {
        return parseToJson(input);
    }
}
