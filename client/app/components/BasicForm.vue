<script setup lang="ts">
import BackArrowIcon from '~/assets/icons/back-arrow-icon.svg';

const emit = defineEmits(['submit', 'back']);
const props = defineProps<{
	header?: string;
	isLoading?: boolean;
	withBackButton?: boolean;
	backHref?: string;
	validationSchema?: Record<string, any>;
	initialValues?: Record<string, any>;
}>();
</script>

<template>
	<Form
		class="container"
		:validation-schema="props.validationSchema"
		:initial-values="props.initialValues"
		@submit="emit('submit', $event)"
	>
		<div class="backButtonContainer">
			<IconButton
				v-if="props.withBackButton"
				:type="props.backHref ? 'link' : 'button'"
				:href="props.backHref"
				@click="emit('back', $event)"
			>
				<BackArrowIcon />
			</IconButton>
		</div>
		<div
			v-if="props.header"
			class="head"
		>
			<h1>{{ props.header }}</h1>
		</div>
		<div
			class="subheader"
		>
			<h2><slot name="subheader" /></h2>
		</div>
		<div class="content">
			<slot />
		</div>
		<div class="actionsContainer">
			<slot name="actions" />
		</div>
	</Form>
</template>

<style scoped lang="scss">
.container {
    box-sizing: border-box;
    width: $desktop_form_width;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    background: $glass_card_background;
    border-radius: $border_radius;
    box-shadow: $shadow_l1;
    backdrop-filter: $blur;
    -webkit-backdrop-filter: $blur;
    border: $glass_border;
    padding: $input_padding;

    & h1 {
        margin-top: 0;
        color: $text_color;
    }
}

.head {
    width: 100%;
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    margin-bottom: 24px;

    & h1 {
        margin: 0;
        text-align: center;
        padding: 0 48px;
        color: $text_color;
    }
}

.backButtonContainer {
    z-index: 1;
    position: fixed;
    left: 20px;
    top: 20px;
}

.subheader {
    color: $subheader_color;
    max-width: 300px;
    font-size: 11px;
    margin-bottom: 16px;

    & h2 {
        margin: 0;
        font-weight: 400;
        line-height: 20px;
        text-align: center;
    }
}

.content {
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 16px;
}

.actionsContainer {
    margin-top: 16px;
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 12px;
}

@media (max-width: $mobile_width) {
    .container {
        width: 100%;
        height: 100%;
        padding: 0 24px 0 24px;
        border-radius: 0;
        overflow: hidden;
    }
}
</style>
