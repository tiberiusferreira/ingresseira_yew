pub fn setting_svg(class: &str, fill_color: &str) ->  String{
    format!(r#"<svg class="{}" width="100%" height="100%" version="1.1" id="Capa_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px"
    viewBox="0 0 478.703 478.703" style="enable-background:new 0 0 478.703 478.703;" xml:space="preserve">
        <g>
        <path fill="{}" d="M454.2,189.101l-33.6-5.7c-3.5-11.3-8-22.2-13.5-32.6l19.8-27.7c8.4-11.8,7.1-27.9-3.2-38.1l-29.8-29.8
			c-5.6-5.6-13-8.7-20.9-8.7c-6.2,0-12.1,1.9-17.1,5.5l-27.8,19.8c-10.8-5.7-22.1-10.4-33.8-13.9l-5.6-33.2
			c-2.4-14.3-14.7-24.7-29.2-24.7h-42.1c-14.5,0-26.8,10.4-29.2,24.7l-5.8,34c-11.2,3.5-22.1,8.1-32.5,13.7l-27.5-19.8
			c-5-3.6-11-5.5-17.2-5.5c-7.9,0-15.4,3.1-20.9,8.7l-29.9,29.8c-10.2,10.2-11.6,26.3-3.2,38.1l20,28.1
			c-5.5,10.5-9.9,21.4-13.3,32.7l-33.2,5.6c-14.3,2.4-24.7,14.7-24.7,29.2v42.1c0,14.5,10.4,26.8,24.7,29.2l34,5.8
			c3.5,11.2,8.1,22.1,13.7,32.5l-19.7,27.4c-8.4,11.8-7.1,27.9,3.2,38.1l29.8,29.8c5.6,5.6,13,8.7,20.9,8.7c6.2,0,12.1-1.9,17.1-5.5
			l28.1-20c10.1,5.3,20.7,9.6,31.6,13l5.6,33.6c2.4,14.3,14.7,24.7,29.2,24.7h42.2c14.5,0,26.8-10.4,29.2-24.7l5.7-33.6
			c11.3-3.5,22.2-8,32.6-13.5l27.7,19.8c5,3.6,11,5.5,17.2,5.5l0,0c7.9,0,15.3-3.1,20.9-8.7l29.8-29.8c10.2-10.2,11.6-26.3,3.2-38.1
			l-19.8-27.8c5.5-10.5,10.1-21.4,13.5-32.6l33.6-5.6c14.3-2.4,24.7-14.7,24.7-29.2v-42.1
			C478.9,203.801,468.5,191.501,454.2,189.101z M451.9,260.401c0,1.3-0.9,2.4-2.2,2.6l-42,7c-5.3,0.9-9.5,4.8-10.8,9.9
			c-3.8,14.7-9.6,28.8-17.4,41.9c-2.7,4.6-2.5,10.3,0.6,14.7l24.7,34.8c0.7,1,0.6,2.5-0.3,3.4l-29.8,29.8c-0.7,0.7-1.4,0.8-1.9,0.8
			c-0.6,0-1.1-0.2-1.5-0.5l-34.7-24.7c-4.3-3.1-10.1-3.3-14.7-0.6c-13.1,7.8-27.2,13.6-41.9,17.4c-5.2,1.3-9.1,5.6-9.9,10.8l-7.1,42
			c-0.2,1.3-1.3,2.2-2.6,2.2h-42.1c-1.3,0-2.4-0.9-2.6-2.2l-7-42c-0.9-5.3-4.8-9.5-9.9-10.8c-14.3-3.7-28.1-9.4-41-16.8
			c-2.1-1.2-4.5-1.8-6.8-1.8c-2.7,0-5.5,0.8-7.8,2.5l-35,24.9c-0.5,0.3-1,0.5-1.5,0.5c-0.4,0-1.2-0.1-1.9-0.8l-29.8-29.8
			c-0.9-0.9-1-2.3-0.3-3.4l24.6-34.5c3.1-4.4,3.3-10.2,0.6-14.8c-7.8-13-13.8-27.1-17.6-41.8c-1.4-5.1-5.6-9-10.8-9.9l-42.3-7.2
			c-1.3-0.2-2.2-1.3-2.2-2.6v-42.1c0-1.3,0.9-2.4,2.2-2.6l41.7-7c5.3-0.9,9.6-4.8,10.9-10c3.7-14.7,9.4-28.9,17.1-42
			c2.7-4.6,2.4-10.3-0.7-14.6l-24.9-35c-0.7-1-0.6-2.5,0.3-3.4l29.8-29.8c0.7-0.7,1.4-0.8,1.9-0.8c0.6,0,1.1,0.2,1.5,0.5l34.5,24.6
			c4.4,3.1,10.2,3.3,14.8,0.6c13-7.8,27.1-13.8,41.8-17.6c5.1-1.4,9-5.6,9.9-10.8l7.2-42.3c0.2-1.3,1.3-2.2,2.6-2.2h42.1
			c1.3,0,2.4,0.9,2.6,2.2l7,41.7c0.9,5.3,4.8,9.6,10,10.9c15.1,3.8,29.5,9.7,42.9,17.6c4.6,2.7,10.3,2.5,14.7-0.6l34.5-24.8
			c0.5-0.3,1-0.5,1.5-0.5c0.4,0,1.2,0.1,1.9,0.8l29.8,29.8c0.9,0.9,1,2.3,0.3,3.4l-24.7,34.7c-3.1,4.3-3.3,10.1-0.6,14.7
			c7.8,13.1,13.6,27.2,17.4,41.9c1.3,5.2,5.6,9.1,10.8,9.9l42,7.1c1.3,0.2,2.2,1.3,2.2,2.6v42.1H451.9z"/>
        <path fill="{}" d="M239.4,136.001c-57,0-103.3,46.3-103.3,103.3s46.3,103.3,103.3,103.3s103.3-46.3,103.3-103.3S296.4,136.001,239.4,136.001
			z M239.4,315.601c-42.1,0-76.3-34.2-76.3-76.3s34.2-76.3,76.3-76.3s76.3,34.2,76.3,76.3S281.5,315.601,239.4,315.601z"/>
        </g>
        </svg>"#, class, fill_color, fill_color)
}

pub fn cone_svg(class: &str, fill_color: &str) ->  String{
    format!(r#"<svg class={} xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 512 512" aria-labelledby="title">
        <path fill={} d="M345.863,281.853c19.152-8.872,38.221-15.344,56.953-19.308c5.403-1.143,8.856-6.45,7.713-11.853
				c-1.143-5.403-6.444-8.857-11.854-7.713c-21.683,4.588-43.704,12.261-65.728,22.865c-12.303-14.673-26.387-30.015-41.582-45.21
				c-16.147-16.146-32.462-31.048-47.961-43.878c13.412-16.959,24.359-35.947,32.641-56.728
				c12.913-32.401,19.461-69.419,19.461-110.029c0-5.523-4.478-10-10-10s-10,4.477-10,10c0,62.572-16.044,114.358-47.728,154.32
				c-5.979-4.561-11.776-8.753-17.312-12.486c-29.637-19.989-47.289-24.674-57.155-15.162c-0.07,0.065-0.143,0.129-0.211,0.196
				c-0.01,0.01-0.02,0.019-0.03,0.028c-0.006,0.006-0.011,0.013-0.017,0.018c-2.527,2.533-4.087,5.596-4.678,9.251L0.778,498.133
				c-1.57,3.744-0.72,8.067,2.151,10.938C4.843,510.984,7.401,512,10.003,512c1.302,0,2.616-0.254,3.865-0.778
				c0.028-0.012,0,0,0.028-0.012l351.941-147.588c3.337-0.54,6.172-1.904,8.569-4.061c0.174-0.145,0.337-0.299,0.5-0.453
				c0.064-0.063,0.133-0.115,0.196-0.179c0.01-0.01,0.018-0.021,0.027-0.03c0.069-0.068,0.133-0.142,0.199-0.213
				c9.511-9.867,4.826-27.518-15.163-57.153C355.94,295.269,351.139,288.675,345.863,281.853z M79.315,440.759
				c-2.087,13.013-6.728,21.567-8.961,25.087l-41.679,17.478l42.329-100.938C79.286,403.147,82.157,423.035,79.315,440.759z
				 M140.851,406.923c-4.032,22.984-14.726,34.063-17.767,36.811l-26.351,11.05c5.858-20.854,7.398-53.957-15.241-97.409
				l29.968-71.459C142.553,344.866,144.886,383.926,140.851,406.923z M176.522,421.323l-21.554,9.04
				c2.054-5.189,3.865-11.099,5.177-17.796c8.004-40.853-5.062-91.365-38.815-150.187l10.82-25.801
				c8.564,10.305,18.322,24.89,22.274,34.086c1.627,3.787,5.316,6.055,9.192,6.055c1.317,0,2.657-0.263,3.944-0.815
				c5.074-2.181,7.419-8.061,5.239-13.136c-5.864-13.644-20.733-34.874-32.107-46.559l16.533-39.425
				c2.896,5.31,6.483,11.128,10.785,17.506c2.962,4.392,6.207,8.945,9.697,13.625c34.207,51.906,33.433,105.035,26.746,140.484
				C197.342,386.105,182.115,413.71,176.522,421.323z M253.575,389.012l-47.474,19.908c6.855-15.036,13.848-34.775,18.004-56.812
				c4.642-24.614,6.683-56.982-1.906-91.935c4.737,5.004,9.61,10.019,14.609,15.018c14.958,14.957,30.056,28.836,44.52,41
				C279.894,363.404,258.239,384.918,253.575,389.012z M288.045,374.557c5.353-11.075,10.08-25.348,12.188-43.175
				c6.036,4.608,11.889,8.842,17.475,12.609c6.381,4.304,12.198,7.889,17.51,10.785L288.045,374.557z M323.217,323.485
				c-21.74-15.403-47.405-37.577-72.266-62.438c-24.86-24.86-47.035-50.525-62.437-72.265
				c-14.895-21.023-18.909-31.758-19.966-36.408c4.65,1.058,15.385,5.072,36.408,19.966c3.118,2.209,6.322,4.57,9.586,7.045
				c-0.701,0.722-1.393,1.451-2.106,2.164c-3.905,3.905-3.905,10.237,0,14.143c1.953,1.952,4.512,2.929,7.071,2.929
				c2.559,0,5.119-0.977,7.071-2.929c1.256-1.256,2.493-2.529,3.716-3.814c15.169,12.539,31.208,27.179,46.929,42.9
				c13.519,13.519,26.23,27.271,37.504,40.504c-1.806,1.003-3.611,2.023-5.416,3.065c-4.783,2.762-6.422,8.878-3.66,13.66
				c1.852,3.208,5.213,5.002,8.67,5.002c1.696,0,3.416-0.433,4.99-1.342c2.776-1.604,5.554-3.143,8.331-4.646
				c4.345,5.518,8.377,10.883,12.018,16.023c14.896,21.023,18.909,31.758,19.966,36.408
				C354.974,342.393,344.239,338.379,323.217,323.485z"/>
        <path fill={} d="M180.884,288.533c-2.267-5.037-8.186-7.283-13.223-5.016c-4.88,2.196-7.14,7.822-5.214,12.751
				c1.514,3.963,5.299,6.43,9.34,6.43c1.096,0,2.21-0.182,3.304-0.564c5.212-1.824,7.96-7.529,6.135-12.742
				C181.131,289.121,181.002,288.795,180.884,288.533z"/>
        <path fill={} d="M373.433,154.691c1.953,1.952,4.512,2.929,7.071,2.929c2.56,0,5.118-0.977,7.071-2.929
				c22.343-22.343,58.542-32.583,84.971-34.716c5.505-0.444,9.607-5.268,9.163-10.772c-0.445-5.505-5.258-9.591-10.772-9.163
				c-29.973,2.419-71.326,14.331-97.504,40.509C369.528,144.454,369.528,150.786,373.433,154.691z"/>
        <path fill={} d="M504.589,101.963c-0.283-0.076-0.571-0.151-0.862-0.224c-5.359-1.336-10.785,1.929-12.12,7.288
				c-1.334,5.359,1.929,10.786,7.288,12.12l0.511,0.132c0.868,0.233,1.74,0.345,2.598,0.345c4.414,0,8.454-2.945,9.652-7.411
				C513.087,108.879,509.924,103.395,504.589,101.963z"/>
        <path fill={} d="M73.504,83.621h10v10c0,5.522,4.477,10,10,10c5.523,0,10-4.478,10-10v-10h10c5.523,0,10-4.478,10-10s-4.477-10-10-10h-10
				v-10c0-5.522-4.477-10-10-10c-5.523,0-10,4.478-10,10v10h-10c-5.523,0-10,4.478-10,10C63.504,79.144,67.981,83.621,73.504,83.621
				z"/>
        <path fill={} d="M307.502,180.62h10.001v9.999c0,5.523,4.478,10,10,10c5.523,0,10-4.477,10-10v-9.999h9.999c5.522,0,10-4.478,10-10
				c0-5.522-4.478-10-10-10h-9.999v-10.001c0-5.522-4.477-10-10-10c-5.522,0-10,4.478-10,10v10.001h-10.001c-5.522,0-10,4.478-10,10
				C297.502,176.143,301.98,180.62,307.502,180.62z"/>
        <path fill={} d="M446.501,417.618h-10v-10c0-5.522-4.478-10-10-10c-5.522,0-10,4.478-10,10v10h-10c-5.522,0-10,4.478-10,10
				c0,5.522,4.478,10,10,10h10v10c0,5.522,4.478,10,10,10c5.522,0,10-4.478,10-10v-10h10c5.522,0,10-4.478,10-10
				C456.501,422.096,452.024,417.618,446.501,417.618z"/>
        <path fill={} d="M389.969,60.001c16.542,0,30-13.458,30-30s-13.458-30-30-30s-30,13.458-30,30S373.427,60.001,389.969,60.001z
				 M389.969,20.001c5.514,0,10,4.486,10,10c0,5.514-4.486,10-10,10s-10-4.486-10-10C379.969,24.487,384.455,20.001,389.969,20.001z
				"/>
        <path fill={} d="M452.501,173.619c-16.542,0-30,13.458-30,30s13.458,30,30,30s30-13.458,30-30S469.042,173.619,452.501,173.619z
				 M452.501,213.619c-5.514,0-10-4.486-10-10s4.486-10,10-10s10,4.486,10,10S458.015,213.619,452.501,213.619z"/>
        <path fill={} d="M198.503,95.62c16.542,0,30-13.458,30-30s-13.458-30-30-30s-30,13.458-30,30S181.961,95.62,198.503,95.62z
				 M198.503,55.62c5.514,0,10,4.486,10,10c0,5.514-4.486,10-10,10s-10-4.486-10-10C188.503,60.107,192.989,55.62,198.503,55.62z"/>
        <path fill={} d="M449.19,294.058c0-5.508-4.492-10-10-10c-5.508,0-10,4.492-10,10s4.492,10,10,10
				C444.698,304.058,449.19,299.565,449.19,294.058z"/>
        <path fill={} d="M335.189,95.06c5.508,0,10-4.492,10-10s-4.492-10-10-10c-5.508,0-10,4.492-10,10
				C325.19,90.567,329.682,95.06,335.189,95.06z"/>
        <path fill={} d="M469.189,359.057c-5.508,0-10.001,4.493-10.001,10.001c0,5.508,4.493,10,10.001,10c5.507,0,10-4.492,10-10
				C479.189,363.55,474.696,359.057,469.189,359.057z"/>
        <path fill={} d="M139.192,30.001c5.508,0,10-4.492,10-10c0-5.508-4.492-10-10-10s-10,4.492-10,10
				C129.192,25.509,133.684,30.001,139.192,30.001z"/>
    </svg>;"#, class, fill_color, fill_color, fill_color, fill_color, fill_color, fill_color,
            fill_color, fill_color, fill_color, fill_color, fill_color, fill_color, fill_color,
            fill_color)
}

pub fn edit_svg(class: &str, fill_color: &str) ->  String{
    format!(r#"<svg class={} xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 64 64" aria-labelledby="title">
        <path fill={} d="m62.463,1.543c-1.017-1.017-2.403-1.542-3.83-1.543-1.43,0.021-2.778,0.591-3.801,1.609l-2.446,2.443c-0.01,0.012-0.015,0.025-0.024,0.035l-31.909,31.819c-0.104,0.104-0.158,0.233-0.234,0.353-0.131,0.152-0.245,0.317-0.327,0.505l-3.254,7.5c-0.324,0.75-0.169,1.62 0.397,2.211 0.392,0.41 0.927,0.631 1.476,0.631 0.241,0 0.486-0.043 0.719-0.131l7.824-2.943c0.217-0.081 0.406-0.209 0.579-0.352 0.126-0.08 0.262-0.14 0.367-0.245l32.035-31.945c0.006-0.006 0.008-0.014 0.015-0.02l2.341-2.33c2.118-2.111 2.15-5.52 0.072-7.597zm-35.905,37.576l-1.777-1.773 29.151-29.069 1.776,1.773-29.15,29.069zm32.95-32.857l-.916,.912-1.784-1.779 .911-.91c0.265-0.264 0.609-0.411 0.972-0.416 0.344-0.008 0.653,0.119 0.883,0.348 0.491,0.49 0.459,1.319-0.066,1.845z"/>
        <path fill={} d="M58.454,22.253c-1.128,0-2.04,0.911-2.04,2.034v33.611c0,1.121-0.915,2.033-2.04,2.033H6.12    c-1.126,0-2.04-0.912-2.04-2.033V9.787c0-1.121,0.914-2.034,2.04-2.034h33.403c1.127,0,2.04-0.911,2.04-2.034    s-0.913-2.034-2.04-2.034H6.12C2.745,3.685,0,6.422,0,9.787v48.111C0,61.263,2.745,64,6.12,64h48.254    c3.374,0,6.12-2.737,6.12-6.102V24.287C60.494,23.164,59.581,22.253,58.454,22.253z"/>
    </svg>"#, class, fill_color, fill_color)
}

pub fn ticket_svg(class: &str, fill_color: &str) ->  String{
    format!(r#"<svg class={} xmlns="http://www.w3.org/2000/svg" width="100%" height="100%" viewBox="0 0 460.298 460.297" aria-labelledby="title">
<title id="title">Ticket Icon</title>
<path fill={} d="M419.304,232.221c0.772,0.333,1.447,0.78,2.146,1.203l6.08-13.941
			c-0.788-0.228-1.577-0.415-2.349-0.756c-9.315-4.064-13.567-14.908-9.51-24.215l-34.311-14.965l37.4,52.502
			C418.939,232.124,419.126,232.148,419.304,232.221z"/>
<path fill={} d="M9.535,242.195l208.181,90.78L90.617,154.561l-4.715-6.617l6.617-4.715
			c4.609-3.284,5.69-9.714,2.406-14.323l-4.715-6.625l6.617-4.715l11.99-8.543l6.625-4.715l4.715,6.617
			c1.926,2.699,5.056,4.316,8.381,4.316c2.138,0,4.194-0.658,5.942-1.91c2.235-1.593,3.715-3.959,4.17-6.665
			c0.455-2.707-0.171-5.422-1.764-7.657l-4.715-6.617l6.617-4.715l11.99-8.543l0.122-0.089L91.902,53.319
			c-3.016,6.917-9.779,11.047-16.875,11.047c-2.455,0-4.95-0.496-7.348-1.536l-5.885,13.493c9.315,4.064,13.575,14.9,9.51,24.215
			c-3.016,6.917-9.779,11.047-16.875,11.047c-2.455,0-4.95-0.496-7.348-1.536l-5.885,13.493c9.315,4.064,13.575,14.9,9.51,24.215
			c-3.016,6.917-9.779,11.047-16.875,11.047c-2.455,0-4.95-0.496-7.348-1.536l-5.885,13.493c9.315,4.064,13.575,14.9,9.51,24.215
			c-3.016,6.917-9.779,11.047-16.875,11.047c-2.455,0-4.95-0.496-7.348-1.536L0,217.98C9.34,222.036,13.599,232.88,9.535,242.195z"
/>
<path fill={} d="M139.194,119.957c-3.235,2.309-6.966,3.414-10.657,3.414c-5.747,0-11.413-2.682-14.997-7.722
			l-11.99,8.535c5.893,8.275,3.967,19.761-4.308,25.662l134.959,189.445l9.169,12.868l9.161,12.859l51.657,72.515
			c3.235-2.309,6.966-3.414,10.657-3.414c5.747,0,11.413,2.682,14.997,7.722c0.488,0.683,0.837,1.414,1.219,2.138l12.388-8.828
			c-0.561-0.593-1.138-1.171-1.626-1.853c-5.893-8.275-3.967-19.761,4.308-25.662c3.235-2.309,6.966-3.414,10.657-3.414
			c5.747,0,11.413,2.682,14.997,7.722c0.488,0.683,0.837,1.414,1.219,2.138l12.388-8.828c-0.561-0.593-1.138-1.171-1.626-1.853
			c-5.893-8.275-3.967-19.761,4.308-25.654c3.235-2.309,6.966-3.414,10.657-3.414c5.747,0,11.413,2.682,14.997,7.722
			c0.488,0.683,0.837,1.414,1.219,2.138l12.388-8.828c-0.561-0.593-1.138-1.171-1.626-1.853c-5.893-8.275-3.967-19.761,4.308-25.654
			c3.235-2.309,6.966-3.414,10.657-3.414c5.747,0,11.413,2.682,14.997,7.722c0.488,0.683,0.837,1.414,1.219,2.138l12.388-8.828
			c-0.561-0.593-1.138-1.171-1.626-1.853c-5.893-8.275-3.967-19.761,4.308-25.654l-41.984-58.932l-2.91-4.081l-0.39-0.545
			l-10.835-15.209l-5.69-7.99l-41.309-57.989l-9.161-12.859l-9.169-12.868L265.057,30.291c-3.235,2.309-6.966,3.414-10.657,3.414
			c-5.747,0-11.413-2.682-14.997-7.722l-11.99,8.543c5.893,8.275,3.967,19.761-4.308,25.654c-3.235,2.309-6.966,3.414-10.657,3.414
			c-5.747,0-11.413-2.682-14.997-7.722l-11.99,8.543c2.869,4.032,3.853,8.828,3.186,13.363c-0.398,2.699-1.414,5.275-2.967,7.576
			c-1.195,1.78-2.674,3.406-4.528,4.723c-0.732,0.52-1.553,0.764-2.325,1.162c-2.658,1.374-5.471,2.26-8.332,2.26
			c-5.747,0-11.413-2.682-14.997-7.722l-11.99,8.543C149.395,102.57,147.469,114.064,139.194,119.957z"/>
</svg>;"#, class, fill_color, fill_color, fill_color)
}

