<script setup lang="ts">
import { ref } from 'vue';
import type { RuleExpression } from 'vee-validate';

const props = defineProps<{
	type?: 'text' | 'password';
	placeholder?: string;
	maxLength?: number;
	name: string;
	rules?: RuleExpression<unknown>;
	modelValue?: string;
	disabled?: boolean;
}>();
const emit = defineEmits(['update:modelValue', 'focus', 'blur']);
const isFocused = ref(false);

const { value, errorMessage, setErrors } = useField(() => props.name, props.rules, {
	syncVModel: true,
	validateOnValueUpdate: false,
});

watch(value, () => {
	setErrors([]);
});

function handleFocus() {
	isFocused.value = true;
	emit('focus');
}

function handleBlur() {
	isFocused.value = false;
	emit('blur');
}
</script>

<template>
	<div class="container">
		<div
			:class="['inputContainer', { ['focused']: isFocused, ['error']: errorMessage, ['disabled']: props.disabled }]"
		>
			<input
				v-model="value"
				class="input"
				:type="props.type"
				:placeholder="props.placeholder"
				:maxlength="props.maxLength"
				:disabled="props.disabled"
				@focus="handleFocus"
				@blur="handleBlur"
			>
			<div class="endAdornment">
				<slot name="endAdornment" />
			</div>
		</div>
		<div
			v-if="errorMessage"
			:class="
				['helperText', {
					['focused']: isFocused,
					['error']: errorMessage,
				}]
			"
		>
			{{ errorMessage }}
		</div>
	</div>
</template>

<style scoped lang="scss">
.container {
  width: 100%;
}

.inputContainer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  height: 30px;
  position: relative;
  border-radius: $border_radius;
  box-shadow: $shadow_l2;
  backdrop-filter: $blur;
  -webkit-backdrop-filter: $blur;
  border: $glass_border;
  background: $glass_item_background;
  transition: $transition;
  padding: 10px;

  &.focused {
    background: $glass_item_background_focused;
  }

  &.disabled {
    color: $text_color_disabled;
    background: $glass_item_background_disabled;
    pointer-events: none;
  }

  &.error {
    background: $error_glass_item_background;
    &.focused {
      background: $error_glass_item_background_focused;
    }
    &.disabled {
      background: $error_glass_item_background_disabled;
    }
  }
}

.label {
  position: absolute;
  display: none;
}

.input {
  height: 100%;
  width: 100%;
  padding: 0;
  font-size: 18px;
  border-radius: inherit;
  border: none;
  background: transparent;
  color: $text_color;

  &:focus {
    outline: none;
  }

  &::placeholder {
    color: $placeholder_color;
    opacity: 1;
  }
  &::-webkit-input-placeholder {
    color: $placeholder_color;
  }
  &::-moz-placeholder {
    color: $placeholder_color;
    opacity: 1;
  }
  &:-ms-input-placeholder {
    color: $placeholder_color;
  }
  &::-ms-input-placeholder {
    color: $placeholder_color;
  }

  &.error {
    &::placeholder {
      color: $error_placeholder_color;
      opacity: 1;
    }
    &::-webkit-input-placeholder {
      color: $error_placeholder_color;
    }
    &::-moz-placeholder {
      color: $error_placeholder_color;
      opacity: 1;
    }
    &:-ms-input-placeholder {
      color: $error_placeholder_color;
    }
    &::-ms-input-placeholder {
      color: $error_placeholder_color;
    }
  }
}

.helperText {
  padding: 8px 10px 8px 10px;
  font-size: 15px;
  color: $placeholder_color;
  transition: $transition;

  &.focused {
    color: $text_color;
  }

  &.error {
    color: $error_placeholder_color;

    &.focused {
      color: $error_text_color;
    }
  }
}
</style>
