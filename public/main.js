// Wait for DOM to be fully loaded
document.addEventListener('DOMContentLoaded', function() {
    
    // Get elements
    const payButton = document.getElementById('payButton');
    const checkCallbackButton = document.getElementById('checkCallback');
    const phoneInput = document.getElementById('phone');
    const amountInput = document.getElementById('amount');
    


    // Add event listeners
    if (payButton) {
        payButton.addEventListener('click', initiatePayment);
    } else {
        console.error('❌ Pay button not found!');
    }

    if (checkCallbackButton) {
        checkCallbackButton.addEventListener('click', checkLastCallback);
    } else {
        console.error('❌ Check callback button not found!');
    }

    // Format phone number as user types
    if (phoneInput) {
        phoneInput.addEventListener('input', function(e) {
            let value = e.target.value.replace(/\D/g, '');
            
            // Auto-format to 254 format
            if (value.startsWith('0') && value.length === 10) {
                // Convert 07XXXXXXXX to 2547XXXXXXXX
                value = '254' + value.substring(1);
            } else if (value.startsWith('7') && value.length === 9) {
                // Convert 7XXXXXXXX to 2547XXXXXXXX
                value = '254' + value;
            } else if (value.startsWith('254') && value.length > 12) {
                // Limit to 12 characters for 254 format
                value = value.substring(0, 12);
            }
            
            e.target.value = value;
        });
    }

    // Allow form submission with Enter key
    if (amountInput) {
        amountInput.addEventListener('keypress', function(e) {
            if (e.key === 'Enter') {
                initiatePayment();
            }
        });
    }

    if (phoneInput) {
        phoneInput.focus();
    }
});

async function initiatePayment() {
    
    const phoneInput = document.getElementById('phone');
    const amountInput = document.getElementById('amount');
    const button = document.getElementById('payButton');
    const responseDiv = document.getElementById('response');

    if (!phoneInput || !amountInput || !button) {
        return;
    }

    const phone = phoneInput.value.trim();
    const amount = amountInput.value.trim();


    // Basic validation
    if (!phone) {
        showResponse('Please enter your phone number', 'error');
        phoneInput.focus();
        return;
    }

    if (!amount || amount <= 0) {
        showResponse('Please enter a valid amount', 'error');
        amountInput.focus();
        return;
    }

    // Validate phone number format (should be 12 digits starting with 254)
    const phoneRegex = /^254[17]\d{8}$/;
    if (!phoneRegex.test(phone)) {
        showResponse('Please enter a valid phone number in format 2547XXXXXXXX', 'error');
        phoneInput.focus();
        return;
    }

    // Show loading state
    const originalText = button.textContent;
    button.disabled = true;
    button.textContent = 'Processing...';
    
    if (responseDiv) {
        responseDiv.style.display = 'none';
    }

    try {
        const response = await fetch('/stk-push', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                phone: phone,
                amount: amount
            })
        });

        const data = await response.json();

        if (response.ok && data.success) {
            showResponse(
                `✅ STK Push initiated successfully!<br><br>
                📱 Check your phone (${phone}) to complete the payment<br>
                💰 Amount: KES ${amount}<br>
                📝 Status: ${data.data.ResponseDescription || 'Pending'}`,
                'success'
            );
            
            // Clear form on success
            phoneInput.value = '';
            amountInput.value = '';
        } else {
            const errorMessage = data.error || `HTTP ${response.status}`;
            showResponse(`❌ Failed: ${errorMessage}`, 'error');
        }
    } catch (error) {
        showResponse(`❌ Network error: ${error.message}`, 'error');
    } finally {
        // Reset button
        button.disabled = false;
        button.textContent = originalText;
    }
}

function showResponse(message, type) {
    
    const responseDiv = document.getElementById('response');
    if (!responseDiv) {
        console.error('❌ Response div not found');
        return;
    }
    
    responseDiv.innerHTML = message;
    responseDiv.className = `response-area ${type}`;
    responseDiv.style.display = 'block';
    
    // Scroll to response
    responseDiv.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
}

async function checkLastCallback() {
    
    const callbackData = document.getElementById('callbackData');
    const button = document.getElementById('checkCallback');
    
    if (!callbackData || !button) {
        console.error('❌ Callback elements not found');
        return;
    }
    
    const originalText = button.textContent;
    button.disabled = true;
    button.textContent = 'Loading...';
    callbackData.textContent = 'Loading...';
    
    try {
        const response = await fetch('/last-callback');
        
        if (response.ok) {
            const data = await response.json();
            callbackData.textContent = JSON.stringify(data, null, 2);
        } else {
            const text = await response.text();
            callbackData.textContent = text || 'No callback received yet';
        }
    } catch (error) {
        callbackData.textContent = `Error fetching callback: ${error.message}`;
    } finally {
        button.disabled = false;
        button.textContent = originalText;
    }
}