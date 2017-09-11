mergeInto(LibraryManager.library, {
  js_phone_home: function(arr) {
      return phone_home_callback(arr);
  },
  js_query_local_storage: function() {
      return js_string_to_rust_utf8(query_local_storage_callback());
  }
});
