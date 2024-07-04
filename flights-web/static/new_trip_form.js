document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById('flightForm');
    const steps = form.querySelectorAll('.form-step');
    let currentStep = 0;

    const destinationTypeInputs = form.querySelectorAll('input[name="destinationType"]');

    const citySection = document.getElementById('citySection');
    const cityInput = document.getElementById('cityInput');
    const cityResults = document.getElementById('cityResults');
    let cityId = "";
    const radiusSlider = document.getElementById('radiusSlider');
    const radiusValue = document.getElementById('radiusValue');

    const countrySection = document.getElementById('countrySection');
    const countryInput = document.getElementById('countryInput');
    const countryResults = document.getElementById('countryResults');
    let countryId = "";
    const subdivisionsSection = document.getElementById('subdivisionsSection');
    const subdivisionCheckboxes = document.getElementById('subdivisionCheckboxes');

    const departureSection = document.getElementById('departureSection');
    const departureInput = document.getElementById('departureInput');
    const departureResults = document.getElementById('departureResults');
    let departureId = "";
    const departureRadiusSlider = document.getElementById('departureRadiusSlider');
    const departureRadiusValue = document.getElementById('departureRadiusValue');

    const prevButton = document.getElementById('prevButton');
    const nextButton = document.getElementById('nextButton');


    destinationTypeInputs.forEach(input => {
        input.addEventListener('change', () => {
            citySection.style.display = input.value === 'city' ? 'block' : 'none';
            countrySection.style.display = input.value === 'country' ? 'block' : 'none';
        });
    });

    function showStep(stepIndex) {
        steps.forEach((step, index) => {
            step.style.display = index === stepIndex ? 'block' : 'none';
        });
        prevButton.style.display = stepIndex > 0 ? 'inline-block' : 'none';
        nextButton.textContent = stepIndex === steps.length - 1 ? 'Submit' : 'Next';
    }

    function validateStep(stepIndex) {
        if (stepIndex === 0) {
            return form.querySelector('input[name="destinationType"]:checked') !== null;
        }
        //todo: Add validation for other steps as needed
        return true;
    }

    function debounce(func, delay) {
        let debounceTimer;
        return function () {
            const context = this;
            const args = arguments;
            clearTimeout(debounceTimer);
            debounceTimer = setTimeout(() => func.apply(context, args), delay);
        }
    }

    function searchCities() {
        const cityResultsContainer = document.getElementById('cityResultsContainer');
        if (cityInput.value.length < 2) {
            cityResults.innerHTML = '';
            cityResultsContainer.style.display = 'none';
            return;
        }
        fetch(`/api/locations/cities?term=${cityInput.value}`)
            .then(response => response.json())
            .then(data => {
                cityResults.innerHTML = data.map(city =>
                    `<li><a href="#" class="city-result" data-id="${city.id}">${city.name}, ${city.country_name}</a></li>`
                ).join('');
                cityResultsContainer.style.display = 'block';
            });
    }

    function searchCitiesForDeparture() {
        const container = document.getElementById('departureResultsContainer');
        if (departureInput.value.length < 1) {
            departureResults.innerHTML = '';
            container.style.display = 'none';
            return;
        }
        fetch(`/api/locations/cities?term=${departureInput.value}`)
            .then(response => response.json())
            .then(data => {
                departureResults.innerHTML = data.map(city =>
                    `<li><a href="#" class="departure-result" data-id="${city.id}">${city.name}, ${city.country_name}</a></li>`
                ).join('');
                container.style.display = 'block';
            });
    }

    function searchCountries() {
        const countryResultsContainer = document.getElementById('countryResultsContainer');
        if (countryInput.value.length < 2) {
            countryResults.innerHTML = '';
            countryResultsContainer.style.display = 'none';
            return;
        }
        fetch(`/api/locations/countries?term=${countryInput.value}`)
            .then(response => response.json())
            .then(data => {
                countryResults.innerHTML = data.map(country =>
                    `<li><a href="#" class="country-result" data-id="${country.id}">${country.name}</a></li>`
                ).join('');
                countryResultsContainer.style.display = 'block';
            });
    }

    cityInput.addEventListener('input', debounce(searchCities, 300));
    countryInput.addEventListener('input', debounce(searchCountries, 300));
    departureInput.addEventListener('input', debounce(searchCitiesForDeparture, 300));

    cityResults.addEventListener('click', (e) => {
        if (e.target.classList.contains('city-result')) {
            e.preventDefault();
            cityInput.value = e.target.textContent;
            cityId = e.target.getAttribute('data-id');
            document.getElementById('cityResultsContainer').style.display = 'none';
        }
    });
    departureResults.addEventListener('click', (e) => {
        if (e.target.classList.contains('departure-result')) {
            e.preventDefault();
            departureInput.value = e.target.textContent;
            departureId = e.target.getAttribute('data-id');
            document.getElementById('departureResultsContainer').style.display = 'none';
        }
    });

    countryResults.addEventListener('click', (e) => {
        if (e.target.classList.contains('country-result')) {
            e.preventDefault();
            countryInput.value = e.target.textContent;
            document.getElementById('countryResultsContainer').style.display = 'none';
            loadSubdivisions(e.target.dataset.id);
        }
    });

    function loadSubdivisions(countryId) {
        fetch(`/api/locations/subdivisions?country_id=${countryId}`)
            .then(response => response.json())
            .then(data => {
                subdivisionCheckboxes.innerHTML = data.map(sub =>
                    `<label class="checkbox">
                        <input type="checkbox" name="subdivisions[]" value="${sub.id}" checked>
                        ${sub.name}
                    </label>`
                ).join('');
                subdivisionsSection.style.display = 'block';
            });
    }

    radiusSlider.addEventListener('input', () => {
        radiusValue.textContent = radiusSlider.value;
    });
    departureRadiusSlider.addEventListener('input', () => {
        departureRadiusValue.textContent = departureRadiusSlider.value;
    })

    nextButton.addEventListener('click', () => {
        if (validateStep(currentStep)) {
            if (currentStep < steps.length - 1) {
                currentStep++;
                showStep(currentStep);
            } else {
                // Serialize form data as JSON and send to backend
                let output = {
                    "destination_type": form.querySelector('input[name="destinationType"]:checked').value,
                    "departure_city_id": departureId,
                    "departure_radius": Number(departureRadiusSlider.value),
                    "destination_city_id": cityId,
                    "destination_city_radius": Number(radiusSlider.value),
                    "destination_country_id": countryId,
                    "destination_country_subdivisions": []
                }
                console.log(output);

                fetch('/api/register_trip', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json'
                    },
                    body: JSON.stringify(output)
                })
                    .then(response => {
                        if (response.ok) {
                            window.location.href = '/';
                        } else {
                            alert('Form submission failed.');
                        }
                    });
            }
        } else {
            alert('Please fill out all required fields before proceeding.');
        }
    });

    prevButton.addEventListener('click', () => {
        if (currentStep > 0) {
            currentStep--;
            showStep(currentStep);
        }
    });

    showStep(currentStep);


});