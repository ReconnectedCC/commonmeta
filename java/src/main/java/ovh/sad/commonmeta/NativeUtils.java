package ovh.sad.commonmeta;

import java.io.*;
import java.nio.file.*;

public class NativeUtils {
    protected static void loadLibraryFromResources(String baseName) throws IOException {
        String os = System.getProperty("os.name").toLowerCase();
        String ext;

        if (os.contains("win")) {
            ext = ".dll";
        } else if (os.contains("mac")) {
            ext = ".dylib";
        } else {
            ext = ".so";
        }

        String resourcePath = "/" + libPrefix(os) + baseName + ext;

        try (InputStream in = NativeUtils.class.getResourceAsStream(resourcePath)) {
            if (in == null) {
                throw new FileNotFoundException("Native lib not found in resources: " + resourcePath);
            }

            Path tempFile = Files.createTempFile("native_", ext);
            tempFile.toFile().deleteOnExit();

            Files.copy(in, tempFile, StandardCopyOption.REPLACE_EXISTING);
            System.load(tempFile.toAbsolutePath().toString());
        }
    }

    private static String libPrefix(String os) {
        // Only Linux and macOS use "lib" prefix
        return (os.contains("win")) ? "" : "lib";
    }
}
