# there-is-always-a-reason
WASM demo project about delay reasons for german trains.

<script type="module">
  import init, {alert_reason} from "./pkg/there_is_always_a_reason.js";
  init()
    .then(() => {
      alert_reason()
    });
</script>