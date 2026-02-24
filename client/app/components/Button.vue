<script setup lang="ts">
const attrs = useAttrs();

const emit = defineEmits(['click']);
const props = withDefaults(defineProps<{
	type?: 'button' | 'submit' | 'reset' | 'link';
	color?: 'error' | 'primary' | 'success';
	variant?: 'filled' | 'outlined';
	href?: string;
	stretched?: boolean;
	loading?: boolean;
	disabled?: boolean;
}>(), {
	variant: 'filled',
	color: 'primary',
});

const buttonClasses = computed(() => [
	'button',
	attrs.class,
	{
		['error']: props.color === 'error',
		['success']: props.color === 'success',
		['outlined']: props.variant === 'outlined',
		['disabled']: props.disabled || props.loading,
		['stretched']: props.stretched,
	},
]);

function handleLinkClick(e: Event) {
	if (props.disabled || props.loading) {
		e.preventDefault();
	}
	else {
		emit('click', e);
	}
}
</script>

<template>
	<NuxtLink
		v-if="props.type === 'link'"
		:to="props.href ?? '/'"
		:class="buttonClasses"
		@click="handleLinkClick"
	>
		<Spinner
			v-if="props.loading"
			size="sm"
		/>
		<slot v-else />
	</NuxtLink>
	<button
		v-else
		:type="props.type ?? 'button'"
		:class="buttonClasses"
		:disabled="props.disabled || props.loading"
		@click="emit('click')"
	>
		<Spinner
			v-if="props.loading"
			size="sm"
		/>
		<slot v-else />
	</button>
</template>

<style scoped lang="scss">
.button {
    color: $text_color;
    background: $glass_item_background;
    padding: $button_padding;
    border-radius: $border_radius;
    border: $glass_border;
    box-shadow: $shadow_l2;
    font-size: 16px;
    transition: $transition;
    backdrop-filter: $blur;
    display: flex;
    justify-content: center;
    align-items: center;
    box-sizing: border-box;

    &:hover{
        background-color: $glass_item_background_focused;
        cursor: pointer;
        transform: scale(1.05);
    }

    &.disabled {
        cursor: default;
        background-color: $glass_item_background_disabled;
        transform: scale(1);
        pointer-events: none;
        color: $text_color_disabled;
    }

    &.outlined {
        background: transparent;
        backdrop-filter: none;
        border: none;
        box-shadow: none;

        &.disabled {
            cursor: default;
            color: $text_color_disabled;
        }

        &:hover {
            transform: scale(1);
        }
    }

    &.success {
        background-color: $success_glass_item_background;

        &:hover {
            background-color: $success_glass_item_background_focused;
        }

        &.disabled {
            background-color: $success_glass_item_background_disabled;
        }
    }

    &.stretched {
        width: 100%;
    }
}
</style>
