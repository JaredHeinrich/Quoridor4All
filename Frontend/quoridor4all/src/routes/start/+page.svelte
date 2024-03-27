<script lang="ts">
  import { Input, Label, Button } from 'flowbite-svelte';
</script>

<form>
  <div class="grid gap-6 mb-6 md:grid-cols-2">
    <div>
      <Label for="first_name" class="mb-2">First name</Label>
      <Input type="text" id="first_name" placeholder="John" required />
    </div>
    <div>
      <Button type="submit">Submit</Button>
    </div>
  </div>
</form>